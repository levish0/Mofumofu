use crate::repository::posts::repository_increment_post_view_count;
use crate::utils::redis_cache::{key_exists, set_with_ttl};
use mofumofu_constants::{VIEW_DEDUP_TTL_SECONDS, view_dedup_key};
use mofumofu_errors::errors::ServiceResult;
use redis::aio::ConnectionManager as RedisClient;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_increment_post_view(
    conn: &DatabaseConnection,
    redis_cache: &RedisClient,
    post_id: Uuid,
    anonymous_user_id: &str,
) -> ServiceResult<()> {
    let dedup_key = view_dedup_key(post_id, anonymous_user_id);

    match key_exists(redis_cache, &dedup_key).await {
        Ok(true) => return Ok(()),
        Ok(false) => {}
        Err(e) => {
            tracing::warn!("View dedup Redis check failed, allowing increment: {:?}", e);
        }
    }

    repository_increment_post_view_count(conn, post_id).await?;

    if let Err(e) = set_with_ttl(redis_cache, &dedup_key, "1", VIEW_DEDUP_TTL_SECONDS).await {
        tracing::warn!("Failed to set view dedup key for {}: {:?}", post_id, e);
    }

    Ok(())
}
