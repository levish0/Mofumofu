use crate::service::follows::service_get_followers;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::follows::{FollowListResponse, GetFollowersRequest};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/follows/followers",
    params(GetFollowersRequest),
    responses(
        (status = 200, description = "Followers retrieved successfully", body = FollowListResponse),
        (status = 400, description = "Bad request - Invalid query parameters"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Follows"
)]
pub async fn get_followers(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<GetFollowersRequest>,
) -> Result<FollowListResponse, Errors> {
    service_get_followers(&state.read_db, payload).await
}
