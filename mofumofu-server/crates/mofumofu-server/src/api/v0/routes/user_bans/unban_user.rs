use crate::extractors::RequiredSession;
use crate::service::user_bans::service_unban_user;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use mofumofu_dto::user_bans::UnbanUserRequest;
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/users/unban",
    request_body = UnbanUserRequest,
    responses(
        (status = 204, description = "User unbanned successfully"),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 403, description = "Forbidden - Insufficient privileges"),
        (status = 404, description = "Not Found - No active ban"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Bans"
)]
pub async fn unban_user(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<UnbanUserRequest>,
) -> Result<StatusCode, Errors> {
    service_unban_user(&state.write_db, session_context.user_id, payload.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
