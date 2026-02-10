use crate::bridge::worker_client::delete_post_from_index;
use crate::repository::hashtags::repository_decrement_hashtag_usage_count;
use crate::repository::post_hashtags::repository_find_post_hashtags_by_post_id;
use crate::repository::posts::{repository_delete_post, repository_get_post_by_id};
use crate::state::WorkerClient;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_delete_post(
    conn: &DatabaseConnection,
    worker: &WorkerClient,
    user_id: Uuid,
    post_id: Uuid,
) -> ServiceResult<()> {
    let txn = conn.begin().await?;

    let post = repository_get_post_by_id(&txn, post_id).await?;

    if post.user_id != user_id {
        return Err(Errors::ForbiddenError(
            "Not the owner of this resource".to_string(),
        ));
    }

    let post_hashtags = repository_find_post_hashtags_by_post_id(&txn, post_id).await?;
    for ph in &post_hashtags {
        repository_decrement_hashtag_usage_count(&txn, ph.hashtag_id).await?;
    }

    repository_delete_post(&txn, post_id).await?;

    txn.commit().await?;

    // Queue search index deletion (best-effort, don't fail the request)
    if let Err(e) = delete_post_from_index(worker, post_id).await {
        tracing::warn!(
            "Failed to queue post delete index job for {}: {:?}",
            post_id,
            e
        );
    }

    Ok(())
}
