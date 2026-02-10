use crate::repository::comments::repository_get_comment_by_id;
use crate::repository::comments::repository_increment_comment_like_count;
use crate::repository::likes::{repository_create_like, repository_find_like};
use crate::repository::posts::repository_get_post_by_id;
use crate::repository::posts::repository_increment_post_like_count;
use mofumofu_dto::likes::LikeStatusResponse;
use mofumofu_entity::common::LikeTargetType;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_create_like(
    conn: &DatabaseConnection,
    user_id: Uuid,
    target_type: LikeTargetType,
    target_id: Uuid,
) -> ServiceResult<LikeStatusResponse> {
    let txn = conn.begin().await?;

    match target_type {
        LikeTargetType::Post => {
            repository_get_post_by_id(&txn, target_id)
                .await
                .map_err(|_| Errors::LikeTargetNotFound)?;
        }
        LikeTargetType::Comment => {
            repository_get_comment_by_id(&txn, target_id)
                .await
                .map_err(|_| Errors::LikeTargetNotFound)?;
        }
    }

    let existing = repository_find_like(&txn, user_id, target_type.clone(), target_id).await?;
    if existing.is_some() {
        return Err(Errors::LikeAlreadyLiked);
    }

    repository_create_like(&txn, user_id, target_type.clone(), target_id).await?;

    match target_type {
        LikeTargetType::Post => {
            repository_increment_post_like_count(&txn, target_id).await?;
        }
        LikeTargetType::Comment => {
            repository_increment_comment_like_count(&txn, target_id).await?;
        }
    }

    txn.commit().await?;

    Ok(LikeStatusResponse { liked: true })
}
