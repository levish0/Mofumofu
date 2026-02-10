use crate::service::user_roles::service_get_user_roles;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use mofumofu_dto::user_roles::{UserIdPath, UserRoleResponse};
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/users/{user_id}/roles",
    params(UserIdPath),
    responses(
        (status = 200, description = "User roles retrieved successfully", body = Vec<UserRoleResponse>),
        (status = 400, description = "Bad request - Invalid path parameter"),
        (status = 404, description = "Not Found - User does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "User Roles"
)]
pub async fn get_user_roles(
    State(state): State<AppState>,
    ValidatedPath(path): ValidatedPath<UserIdPath>,
) -> Result<Json<Vec<UserRoleResponse>>, Errors> {
    let response = service_get_user_roles(&state.read_db, path.user_id).await?;
    Ok(Json(response))
}
