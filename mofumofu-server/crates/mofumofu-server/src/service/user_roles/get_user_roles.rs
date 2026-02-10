use crate::repository::user_roles::repository_find_user_roles;
use mofumofu_dto::user_roles::UserRoleResponse;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn service_get_user_roles(
    conn: &DatabaseConnection,
    user_id: Uuid,
) -> ServiceResult<Vec<UserRoleResponse>> {
    let roles = repository_find_user_roles(conn, user_id).await?;

    Ok(roles.into_iter().map(UserRoleResponse::from).collect())
}
