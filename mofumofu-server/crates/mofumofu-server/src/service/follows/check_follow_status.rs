use crate::repository::follows::repository_find_follow;
use mofumofu_dto::follows::FollowStatusResponse;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_check_follow_status(
    conn: &DatabaseConnection,
    follower_id: Uuid,
    followee_id: Uuid,
) -> ServiceResult<FollowStatusResponse> {
    let follow = repository_find_follow(conn, follower_id, followee_id).await?;

    Ok(FollowStatusResponse {
        following: follow.is_some(),
    })
}
