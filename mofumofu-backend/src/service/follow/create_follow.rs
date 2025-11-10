use crate::dto::follow::internal::create::CreateFollow;
use crate::entity::common::{ActionType, TargetType};
use crate::repository::follow::check_follow_exists::repository_check_follow_exists;
use crate::repository::follow::create_follow::repository_create_follow;
use crate::repository::system_events::log_event::repository_log_event;
use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::repository::user::update_follow_count::{
    repository_increment_user_follower_count, repository_increment_user_following_count,
};
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession};
use sea_orm::TransactionTrait;

pub async fn service_create_follow_by_handle<C>(
    conn: &C,
    payload: CreateFollow,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    let follower = repository_find_user_by_uuid(&txn, &payload.follower_id)
        .await?
        .ok_or(Errors::UserNotFound)?;
    let followee = repository_find_user_by_handle(&txn, &payload.followee_handle)
        .await?
        .ok_or(Errors::UserNotFound)?;

    if follower.handle == followee.handle {
        return Err(Errors::FollowCannotFollowSelf);
    }

    // 이미 팔로우 관계가 있는지 체크
    let follow_exists = repository_check_follow_exists(&txn, follower.id, followee.id).await?;

    if follow_exists {
        return Err(Errors::FollowAlreadyFollowing);
    }

    // Insert the new follow relationship
    let _created_follow = repository_create_follow(&txn, follower.id, followee.id).await?;

    // Update follow counts
    repository_increment_user_following_count(&txn, follower.id).await?;
    repository_increment_user_follower_count(&txn, followee.id).await?;

    // Commit the transaction
    txn.commit().await?;

    // 팔로우 생성 이벤트 로깅
    repository_log_event(
        conn,
        Some(follower.id),
        ActionType::FollowCreated,
        Some(followee.id),
        Some(TargetType::User),
        None,
    )
    .await;

    Ok(())
}
