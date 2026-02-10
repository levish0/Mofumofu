use crate::repository::user_bans::repository_find_user_bans;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_check_active_ban(
    conn: &DatabaseConnection,
    user_id: Uuid,
) -> ServiceResult<bool> {
    let bans = repository_find_user_bans(conn, user_id).await?;

    let has_active_ban = bans.iter().any(|b| {
        b.expires_at
            .map(|exp| exp > chrono::Utc::now())
            .unwrap_or(true)
    });

    Ok(has_active_ban)
}
