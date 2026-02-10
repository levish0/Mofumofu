use crate::service::hashtags::service_get_trending_hashtags;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::hashtags::{TrendingHashtagsResponse, TrendingQuery};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/hashtags/trending",
    params(TrendingQuery),
    responses(
        (status = 200, description = "Trending hashtags retrieved successfully", body = TrendingHashtagsResponse),
        (status = 400, description = "Bad request - Invalid query parameters"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Hashtags"
)]
pub async fn get_trending(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<TrendingQuery>,
) -> Result<TrendingHashtagsResponse, Errors> {
    let limit = payload.limit.unwrap_or(10);
    service_get_trending_hashtags(&state.read_db, limit).await
}
