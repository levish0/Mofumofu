use crate::extractors::RequiredSession;
use crate::service::user_bans::service_get_user_bans;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use mofumofu_dto::user_bans::UserBanResponse;
use mofumofu_dto::user_roles::UserIdPath;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/users/{user_id}/bans",
    params(UserIdPath),
    responses(
        (status = 200, description = "User bans retrieved successfully", body = Vec<UserBanResponse>),
        (status = 400, description = "Bad request - Invalid path parameter"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Bans"
)]
pub async fn get_user_bans(
    State(state): State<AppState>,
    RequiredSession(_session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<UserIdPath>,
) -> Result<Json<Vec<UserBanResponse>>, Errors> {
    let response = service_get_user_bans(&state.read_db, path.user_id).await?;
    Ok(Json(response))
}
