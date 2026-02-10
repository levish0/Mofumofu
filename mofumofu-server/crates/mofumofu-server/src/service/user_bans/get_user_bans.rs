use crate::repository::user_bans::repository_find_user_bans;
use mofumofu_dto::user_bans::UserBanResponse;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_get_user_bans(
    conn: &DatabaseConnection,
    user_id: Uuid,
) -> ServiceResult<Vec<UserBanResponse>> {
    let bans = repository_find_user_bans(conn, user_id).await?;

    Ok(bans.into_iter().map(UserBanResponse::from).collect())
}
