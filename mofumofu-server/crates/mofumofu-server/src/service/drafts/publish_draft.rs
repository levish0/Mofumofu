use crate::bridge::markdown_client::render_markdown;
use crate::repository::drafts::{repository_delete_draft, repository_get_draft_by_id};
use crate::repository::hashtags::{
    repository_find_or_create_hashtag, repository_increment_hashtag_usage_count,
};
use crate::repository::post_hashtags::repository_create_post_hashtag;
use crate::repository::posts::{PostUpdateParams, repository_create_post, repository_update_post};
use mofumofu_dto::drafts::PublishDraftRequest;
use mofumofu_dto::posts::PostResponse;
use mofumofu_errors::errors::{Errors, ServiceResult};
use reqwest::Client as HttpClient;
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_publish_draft(
    conn: &DatabaseConnection,
    http_client: &HttpClient,
    user_id: Uuid,
    draft_id: Uuid,
    payload: PublishDraftRequest,
) -> ServiceResult<PostResponse> {
    // Pre-read draft for content (needed for external markdown render call)
    let draft = repository_get_draft_by_id(conn, draft_id).await?;

    let title = draft.title.ok_or(Errors::DraftMissingTitle)?;
    let content = draft.content.ok_or(Errors::DraftMissingContent)?;
    let slug = payload
        .slug
        .or(draft.slug)
        .unwrap_or_else(|| slug_from_title(&title));

    // Render markdown before transaction (external HTTP call)
    let (html, toc) = render_markdown(http_client, &content).await?;

    let txn = conn.begin().await?;

    // Re-verify ownership inside transaction (TOCTOU safety)
    let draft = repository_get_draft_by_id(&txn, draft_id).await?;
    if draft.user_id != user_id {
        return Err(Errors::ForbiddenError(
            "Not the owner of this resource".to_string(),
        ));
    }

    let post = repository_create_post(
        &txn,
        user_id,
        title,
        slug,
        content,
        payload.summary,
        payload.thumbnail_image,
    )
    .await?;

    let update_params = PostUpdateParams {
        render: Some(Some(html)),
        toc: Some(Some(toc)),
        published_at: Some(Some(chrono::Utc::now())),
        ..Default::default()
    };
    let post = repository_update_post(&txn, post.id, update_params).await?;

    let hashtag_names = payload.hashtags.unwrap_or_default();
    for tag_name in &hashtag_names {
        let hashtag = repository_find_or_create_hashtag(&txn, tag_name.clone()).await?;
        repository_create_post_hashtag(&txn, post.id, hashtag.id).await?;
        repository_increment_hashtag_usage_count(&txn, hashtag.id).await?;
    }

    if payload.delete_draft.unwrap_or(true) {
        repository_delete_draft(&txn, draft_id).await?;
    }

    txn.commit().await?;

    Ok(PostResponse::from_model(post, hashtag_names))
}

fn slug_from_title(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}
