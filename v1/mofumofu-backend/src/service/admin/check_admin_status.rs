use crate::service::auth::require_admin;
use crate::service::error::errors::Errors;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_check_admin_status(
    conn: &DatabaseConnection,
    user_id: Uuid,
) -> Result<bool, Errors> {
    match require_admin(conn, user_id).await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
