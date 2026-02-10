use crate::repository::hashtags::repository_decrement_hashtag_usage_count;
use crate::repository::post_hashtags::repository_find_post_hashtags_by_post_id;
use crate::repository::posts::{repository_delete_post, repository_get_post_by_id};
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_delete_post(
    conn: &DatabaseConnection,
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

    Ok(())
}
