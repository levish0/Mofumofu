use crate::repository::follows::{repository_create_follow, repository_find_follow};
use crate::repository::user::{
    repository_get_user_by_id, repository_increment_user_follower_count,
    repository_increment_user_following_count,
};
use mofumofu_dto::follows::FollowStatusResponse;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_create_follow(
    conn: &DatabaseConnection,
    follower_id: Uuid,
    followee_id: Uuid,
) -> ServiceResult<FollowStatusResponse> {
    if follower_id == followee_id {
        return Err(Errors::FollowSelfFollow);
    }

    let txn = conn.begin().await?;

    repository_get_user_by_id(&txn, followee_id).await?;

    let existing = repository_find_follow(&txn, follower_id, followee_id).await?;
    if existing.is_some() {
        return Err(Errors::FollowAlreadyFollowing);
    }

    repository_create_follow(&txn, follower_id, followee_id).await?;
    repository_increment_user_following_count(&txn, follower_id).await?;
    repository_increment_user_follower_count(&txn, followee_id).await?;

    txn.commit().await?;

    Ok(FollowStatusResponse { following: true })
}
