use crate::bridge::worker_client::{delete_post_from_index, index_post};
use crate::state::WorkerClient;
use crate::utils::redis_cache::{delete_key, set_json_compressed};
use mofumofu_constants::{POST_RENDER_CACHE_TTL_SECONDS, post_render_key};
use mofumofu_dto::posts::CachedPostRender;
use redis::aio::ConnectionManager as RedisClient;
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Performs post-commit processing after creating or updating a post:
/// 1. Invalidates old render cache (no-op if key doesn't exist)
/// 2. Caches rendered HTML + TOC in Redis (zstd compressed, 30 days)
/// 3. Indexes post in MeiliSearch (fire-and-forget)
///
/// All operations are best-effort — failures are logged but don't propagate.
pub async fn post_process_post(
    worker: &WorkerClient,
    redis_cache: &RedisClient,
    post_id: Uuid,
    render: &str,
    toc: &JsonValue,
) {
    let cache_key = post_render_key(post_id);

    if let Err(e) = delete_key(redis_cache, &cache_key).await {
        tracing::warn!("Failed to delete render cache for {}: {:?}", post_id, e);
    }

    let cached = CachedPostRender {
        render: render.to_string(),
        toc: toc.clone(),
    };
    if let Err(e) = set_json_compressed(
        redis_cache,
        &cache_key,
        &cached,
        POST_RENDER_CACHE_TTL_SECONDS,
    )
    .await
    {
        tracing::warn!("Failed to cache post render for {}: {:?}", post_id, e);
    }

    if let Err(e) = index_post(worker, post_id).await {
        tracing::warn!("Failed to queue post index job for {}: {:?}", post_id, e);
    }
}

/// Performs post-commit cleanup after deleting a post:
/// 1. Invalidates render cache
/// 2. Removes post from MeiliSearch index (fire-and-forget)
pub async fn post_process_post_delete(
    worker: &WorkerClient,
    redis_cache: &RedisClient,
    post_id: Uuid,
) {
    if let Err(e) = delete_key(redis_cache, &post_render_key(post_id)).await {
        tracing::warn!("Failed to delete render cache for {}: {:?}", post_id, e);
    }

    if let Err(e) = delete_post_from_index(worker, post_id).await {
        tracing::warn!(
            "Failed to queue post delete index job for {}: {:?}",
            post_id,
            e
        );
    }
}
