use crate::service::follows::service_get_following;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::follows::{FollowListResponse, GetFollowingRequest};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/follows/following",
    params(GetFollowingRequest),
    responses(
        (status = 200, description = "Following list retrieved successfully", body = FollowListResponse),
        (status = 400, description = "Bad request - Invalid query parameters"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Follows"
)]
pub async fn get_following(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<GetFollowingRequest>,
) -> Result<FollowListResponse, Errors> {
    service_get_following(&state.read_db, payload).await
}
