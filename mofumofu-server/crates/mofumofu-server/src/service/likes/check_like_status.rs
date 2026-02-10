use crate::repository::likes::repository_find_like;
use mofumofu_dto::likes::LikeStatusResponse;
use mofumofu_entity::common::LikeTargetType;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_check_like_status(
    conn: &DatabaseConnection,
    user_id: Uuid,
    target_type: LikeTargetType,
    target_id: Uuid,
) -> ServiceResult<LikeStatusResponse> {
    let like = repository_find_like(conn, user_id, target_type, target_id).await?;

    Ok(LikeStatusResponse {
        liked: like.is_some(),
    })
}
