use crate::extractors::RequiredSession;
use crate::service::likes::service_delete_like;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::likes::{LikeRequest, LikeStatusResponse};
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    delete,
    path = "/v0/likes",
    request_body = LikeRequest,
    responses(
        (status = 200, description = "Like removed successfully", body = LikeStatusResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 404, description = "Not Found - Like does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Likes"
)]
pub async fn delete_like(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<LikeRequest>,
) -> Result<LikeStatusResponse, Errors> {
    service_delete_like(
        &state.write_db,
        session_context.user_id,
        payload.target_type,
        payload.target_id,
    )
    .await
}
