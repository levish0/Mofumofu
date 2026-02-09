use crate::dto::follow::request::get_count::GetFollowCountRequest;
use crate::dto::follow::response::follow_count::FollowCountResponse;
use crate::service::error::errors::Errors;
use crate::service::follow::get_follower_count::service_get_follower_count;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;

#[utoipa::path(
    post,
    path = "/v0/follow/follower-count",
    request_body = GetFollowCountRequest,
    responses(
        (status = StatusCode::OK, description = "Successfully retrieved follower count", body = FollowCountResponse),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
    ),
    tag = "Follow"
)]
pub async fn api_get_follower_count(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<GetFollowCountRequest>,
) -> Result<FollowCountResponse, Errors> {
    let count = service_get_follower_count(&state.conn, &payload.user_handle).await?;

    Ok(FollowCountResponse { count })
}
