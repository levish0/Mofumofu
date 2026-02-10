use crate::bridge::markdown_client::render_markdown;
use crate::repository::hashtags::{
    repository_find_or_create_hashtag, repository_increment_hashtag_usage_count,
};
use crate::repository::post_hashtags::repository_create_post_hashtag;
use crate::repository::posts::{PostUpdateParams, repository_create_post, repository_update_post};
use crate::repository::user::repository_get_user_by_id;
use crate::state::WorkerClient;
use crate::utils::r2_url::build_r2_public_url;
use mofumofu_dto::posts::{CreatePostRequest, PostAuthor, PostResponse};
use mofumofu_errors::errors::ServiceResult;
use redis::aio::ConnectionManager as RedisClient;
use reqwest::Client as HttpClient;
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

use super::utils::post_process_post;

pub async fn service_create_post(
    conn: &DatabaseConnection,
    http_client: &HttpClient,
    worker: &WorkerClient,
    redis_cache: &RedisClient,
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
        render: Some(Some(html.clone())),
        toc: Some(Some(toc.clone())),
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

    // Post-commit: cache render + index (best-effort)
    post_process_post(worker, redis_cache, post.id, &html, &toc).await;

    let user = repository_get_user_by_id(conn, user_id).await?;
    let author = PostAuthor {
        id: user.id,
        handle: user.handle,
        display_name: user.display_name,
        profile_image: user.profile_image.as_deref().map(build_r2_public_url),
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
