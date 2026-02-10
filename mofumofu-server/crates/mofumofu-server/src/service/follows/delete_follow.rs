use crate::repository::follows::{repository_delete_follow, repository_find_follow};
use crate::repository::user::{
    repository_decrement_user_follower_count, repository_decrement_user_following_count,
};
use mofumofu_dto::follows::FollowStatusResponse;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_delete_follow(
    conn: &DatabaseConnection,
    follower_id: Uuid,
    followee_id: Uuid,
) -> ServiceResult<FollowStatusResponse> {
    let txn = conn.begin().await?;

    let existing = repository_find_follow(&txn, follower_id, followee_id).await?;
    if existing.is_none() {
        return Err(Errors::FollowNotFollowing);
    }

    repository_delete_follow(&txn, follower_id, followee_id).await?;
    repository_decrement_user_following_count(&txn, follower_id).await?;
    repository_decrement_user_follower_count(&txn, followee_id).await?;

    txn.commit().await?;

    Ok(FollowStatusResponse { following: false })
}
