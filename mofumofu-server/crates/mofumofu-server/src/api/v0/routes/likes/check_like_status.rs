use crate::extractors::RequiredSession;
use crate::service::likes::service_check_like_status;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::likes::{LikeRequest, LikeStatusResponse};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/likes/status",
    params(LikeRequest),
    responses(
        (status = 200, description = "Like status retrieved successfully", body = LikeStatusResponse),
        (status = 400, description = "Bad request - Invalid query parameters"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Likes"
)]
pub async fn check_like_status(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedQuery(payload): ValidatedQuery<LikeRequest>,
) -> Result<LikeStatusResponse, Errors> {
    service_check_like_status(
        &state.read_db,
        session_context.user_id,
        payload.target_type,
        payload.target_id,
    )
    .await
}
