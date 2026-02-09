use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::follow::request::check_follow_status::CheckFollowStatusRequest;
use crate::dto::follow::response::follow_status::FollowStatusResponse;
use crate::service::error::errors::Errors;
use crate::service::follow::check_follow_status::service_check_follow_status;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;

#[utoipa::path(
    post,
    path = "/v0/follow/status",
    request_body = CheckFollowStatusRequest,
    responses(
        (status = StatusCode::OK, description = "Follow status retrieved successfully", body = FollowStatusResponse),
        (status = StatusCode::BAD_REQUEST, description = "Bad request"),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Follow"
)]
pub async fn api_check_follow_status(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<CheckFollowStatusRequest>,
) -> Result<FollowStatusResponse, Errors> {
    let user_uuid = claims.sub.clone();

    let is_following =
        service_check_follow_status(&state.conn, &user_uuid, &payload.handle).await?;

    Ok(FollowStatusResponse { is_following })
}
