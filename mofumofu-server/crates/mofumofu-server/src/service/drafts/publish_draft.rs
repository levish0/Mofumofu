use crate::bridge::markdown_client::render_markdown;
use crate::bridge::worker_client::index_post;
use crate::repository::drafts::{repository_delete_draft, repository_get_draft_by_id};
use crate::repository::hashtags::{
    repository_find_or_create_hashtag, repository_increment_hashtag_usage_count,
};
use crate::repository::post_hashtags::repository_create_post_hashtag;
use crate::repository::posts::{PostUpdateParams, repository_create_post, repository_update_post};
use crate::repository::user::repository_get_user_by_id;
use crate::state::WorkerClient;
use mofumofu_dto::drafts::PublishDraftRequest;
use mofumofu_dto::posts::{PostAuthor, PostResponse};
use mofumofu_errors::errors::{Errors, ServiceResult};
use reqwest::Client as HttpClient;
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_publish_draft(
    conn: &DatabaseConnection,
    http_client: &HttpClient,
    worker: &WorkerClient,
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

    // Queue search index job (best-effort, don't fail the request)
    if let Err(e) = index_post(worker, post.id).await {
        tracing::warn!("Failed to queue post index job for {}: {:?}", post.id, e);
    }

    let user = repository_get_user_by_id(conn, user_id).await?;
    let author = PostAuthor {
        id: user.id,
        handle: user.handle,
        display_name: user.display_name,
        profile_image: user.profile_image,
    };

    Ok(PostResponse {
        id: post.id,
        user_id: post.user_id,
        author,
        title: post.title,
        slug: post.slug,
        thumbnail_image: post.thumbnail_image,
        summary: post.summary,
        content: post.content,
        render: post.render,
        toc: post.toc,
        like_count: post.like_count,
        comment_count: post.comment_count,
        view_count: post.view_count,
        hashtags: hashtag_names,
        published_at: post.published_at,
        created_at: post.created_at,
        updated_at: post.updated_at,
    })
}

fn slug_from_title(title: &str) -> String {
    let mut result = String::with_capacity(title.len());
    let mut prev_hyphen = true; // suppress leading hyphen
    for c in title.to_lowercase().chars() {
        if c.is_alphanumeric() {
            result.push(c);
            prev_hyphen = false;
        } else if !prev_hyphen {
            result.push('-');
            prev_hyphen = true;
        }
    }
    result.trim_end_matches('-').to_string()
}
