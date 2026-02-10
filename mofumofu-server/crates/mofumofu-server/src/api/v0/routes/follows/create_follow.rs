use crate::extractors::RequiredSession;
use crate::service::follows::service_create_follow;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::follows::{FollowRequest, FollowStatusResponse};
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/follows",
    request_body = FollowRequest,
    responses(
        (status = 200, description = "Follow created successfully", body = FollowStatusResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 409, description = "Conflict - Already following"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Follows"
)]
pub async fn create_follow(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<FollowRequest>,
) -> Result<FollowStatusResponse, Errors> {
    service_create_follow(
        &state.write_db,
        session_context.user_id,
        payload.followee_id,
    )
    .await
}
