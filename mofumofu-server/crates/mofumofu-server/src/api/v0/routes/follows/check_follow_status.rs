use crate::extractors::RequiredSession;
use crate::service::follows::service_check_follow_status;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::follows::{FollowRequest, FollowStatusResponse};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/follows/status",
    params(FollowRequest),
    responses(
        (status = 200, description = "Follow status retrieved successfully", body = FollowStatusResponse),
        (status = 400, description = "Bad request - Invalid query parameters"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Follows"
)]
pub async fn check_follow_status(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedQuery(payload): ValidatedQuery<FollowRequest>,
) -> Result<FollowStatusResponse, Errors> {
    service_check_follow_status(&state.read_db, session_context.user_id, payload.followee_id).await
}
