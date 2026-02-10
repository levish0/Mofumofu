use crate::repository::comments::repository_decrement_comment_like_count;
use crate::repository::likes::{repository_delete_like, repository_find_like};
use crate::repository::posts::repository_decrement_post_like_count;
use mofumofu_dto::likes::LikeStatusResponse;
use mofumofu_entity::common::LikeTargetType;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_delete_like(
    conn: &DatabaseConnection,
    user_id: Uuid,
    target_type: LikeTargetType,
    target_id: Uuid,
) -> ServiceResult<LikeStatusResponse> {
    let txn = conn.begin().await?;

    let existing = repository_find_like(&txn, user_id, target_type.clone(), target_id).await?;
    if existing.is_none() {
        return Err(Errors::LikeNotLiked);
    }

    repository_delete_like(&txn, user_id, target_type.clone(), target_id).await?;

    match target_type {
        LikeTargetType::Post => {
            repository_decrement_post_like_count(&txn, target_id).await?;
        }
        LikeTargetType::Comment => {
            repository_decrement_comment_like_count(&txn, target_id).await?;
        }
    }

    txn.commit().await?;

    Ok(LikeStatusResponse { liked: false })
}
