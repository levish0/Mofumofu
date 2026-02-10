use crate::extractors::RequiredSession;
use crate::service::user_bans::service_ban_user;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mofumofu_dto::user_bans::BanUserRequest;
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/users/ban",
    request_body = BanUserRequest,
    responses(
        (status = 201, description = "User banned successfully", body = mofumofu_dto::user_bans::UserBanResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 403, description = "Forbidden - Insufficient privileges"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Bans"
)]
pub async fn ban_user(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<BanUserRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_ban_user(&state.write_db, session_context.user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
