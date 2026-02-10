use crate::bridge::markdown_client::render_markdown;
use crate::repository::hashtags::{
    repository_decrement_hashtag_usage_count, repository_find_or_create_hashtag,
    repository_increment_hashtag_usage_count,
};
use crate::repository::post_hashtags::{
    repository_create_post_hashtag, repository_delete_all_post_hashtags_by_post_id,
    repository_find_post_hashtags_by_post_id,
};
use crate::repository::posts::{
    PostUpdateParams, repository_get_post_by_id, repository_update_post,
};
use crate::repository::user::repository_get_user_by_id;
use crate::state::WorkerClient;
use crate::utils::r2_url::build_r2_public_url;
use mofumofu_dto::posts::{PostAuthor, PostResponse, UpdatePostRequest};
use mofumofu_entity::hashtags::Entity as HashtagEntity;
use mofumofu_errors::errors::{Errors, ServiceResult};
use redis::aio::ConnectionManager as RedisClient;
use reqwest::Client as HttpClient;
use sea_orm::{DatabaseConnection, EntityTrait, TransactionTrait};
use uuid::Uuid;

use super::utils::post_process_post;

pub async fn service_update_post(
    conn: &DatabaseConnection,
    http_client: &HttpClient,
    worker: &WorkerClient,
    redis_cache: &RedisClient,
    user_id: Uuid,
    post_id: Uuid,
    payload: UpdatePostRequest,
) -> ServiceResult<PostResponse> {
    // Re-render markdown if content changed (before txn — external HTTP call)
    let render_result = if let Some(ref content) = payload.content {
        Some(render_markdown(http_client, content).await?)
    } else {
        None
    };

    let txn = conn.begin().await?;

    let post = repository_get_post_by_id(&txn, post_id).await?;

    if post.user_id != user_id {
        return Err(Errors::ForbiddenError(
            "Not the owner of this resource".to_string(),
        ));
    }

    let mut update_params = PostUpdateParams {
        title: payload.title,
        slug: payload.slug,
        summary: payload.summary,
        thumbnail_image: payload.thumbnail_image,
        content: payload.content,
        ..Default::default()
    };

    if let Some((ref html, ref toc)) = render_result {
        update_params.render = Some(Some(html.clone()));
        update_params.toc = Some(Some(toc.clone()));
    }

    if payload.publish == Some(true) && post.published_at.is_none() {
        update_params.published_at = Some(Some(chrono::Utc::now()));
    } else if payload.unpublish == Some(true) && post.published_at.is_some() {
        update_params.published_at = Some(None);
    }

    let updated_post = repository_update_post(&txn, post_id, update_params).await?;

    let hashtag_names = if let Some(new_hashtags) = payload.hashtags {
        let old_post_hashtags = repository_find_post_hashtags_by_post_id(&txn, post_id).await?;
        for ph in &old_post_hashtags {
            repository_decrement_hashtag_usage_count(&txn, ph.hashtag_id).await?;
        }
        repository_delete_all_post_hashtags_by_post_id(&txn, post_id).await?;

        for tag_name in &new_hashtags {
            let hashtag = repository_find_or_create_hashtag(&txn, tag_name.clone()).await?;
            repository_create_post_hashtag(&txn, post_id, hashtag.id).await?;
            repository_increment_hashtag_usage_count(&txn, hashtag.id).await?;
        }

        new_hashtags
    } else {
        let post_hashtags = repository_find_post_hashtags_by_post_id(&txn, post_id).await?;
        let mut names = Vec::with_capacity(post_hashtags.len());
        for ph in &post_hashtags {
            if let Some(hashtag) = HashtagEntity::find_by_id(ph.hashtag_id).one(&txn).await? {
                names.push(hashtag.name);
            }
        }
        names
    };

    txn.commit().await?;

    // Post-commit: invalidate old cache, cache new render, index (best-effort)
    if let Some((html, toc)) = render_result {
        post_process_post(worker, redis_cache, post_id, &html, &toc).await;
    }

    let user = repository_get_user_by_id(conn, user_id).await?;
    let author = PostAuthor {
        id: user.id,
        handle: user.handle,
        display_name: user.display_name,
        profile_image: user.profile_image.as_deref().map(build_r2_public_url),
    };

    Ok(PostResponse {
        id: updated_post.id,
        user_id: updated_post.user_id,
        author,
        title: updated_post.title,
        slug: updated_post.slug,
        thumbnail_image: updated_post.thumbnail_image,
        summary: updated_post.summary,
        content: updated_post.content,
        render: updated_post.render,
        toc: updated_post.toc,
        like_count: updated_post.like_count,
        comment_count: updated_post.comment_count,
        view_count: updated_post.view_count,
        hashtags: hashtag_names,
        published_at: updated_post.published_at,
        created_at: updated_post.created_at,
        updated_at: updated_post.updated_at,
    })
}
