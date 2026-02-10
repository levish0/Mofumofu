use crate::bridge::markdown_client::render_markdown;
use crate::repository::hashtags::{
    repository_find_or_create_hashtag, repository_increment_hashtag_usage_count,
};
use crate::repository::post_hashtags::repository_create_post_hashtag;
use crate::repository::posts::{PostUpdateParams, repository_create_post, repository_update_post};
use mofumofu_dto::posts::{CreatePostRequest, PostResponse};
use mofumofu_errors::errors::ServiceResult;
use reqwest::Client as HttpClient;
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_create_post(
    conn: &DatabaseConnection,
    http_client: &HttpClient,
    user_id: Uuid,
    payload: CreatePostRequest,
) -> ServiceResult<PostResponse> {
    // Render markdown before transaction (external HTTP call)
    let (html, toc) = render_markdown(http_client, &payload.content).await?;

    let txn = conn.begin().await?;

    let post = repository_create_post(
        &txn,
        user_id,
        payload.title,
        payload.slug,
        payload.content,
        payload.summary,
        payload.thumbnail_image,
    )
    .await?;

    let published_at = if payload.publish.unwrap_or(false) {
        Some(chrono::Utc::now())
    } else {
        None
    };

    let update_params = PostUpdateParams {
        render: Some(Some(html)),
        toc: Some(Some(toc)),
        published_at: Some(published_at),
        ..Default::default()
    };
    let post = repository_update_post(&txn, post.id, update_params).await?;

    // Handle hashtags
    let hashtag_names = payload.hashtags.unwrap_or_default();
    for tag_name in &hashtag_names {
        let hashtag = repository_find_or_create_hashtag(&txn, tag_name.clone()).await?;
        repository_create_post_hashtag(&txn, post.id, hashtag.id).await?;
        repository_increment_hashtag_usage_count(&txn, hashtag.id).await?;
    }

    txn.commit().await?;

    Ok(PostResponse::from_model(post, hashtag_names))
}
