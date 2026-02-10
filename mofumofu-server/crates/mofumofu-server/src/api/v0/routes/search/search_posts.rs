use crate::service::search::search_posts::service_search_posts;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::search::{SearchPostsRequest, SearchPostsResponse};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/search/posts",
    params(SearchPostsRequest),
    responses(
        (status = 200, description = "Post search results", body = SearchPostsResponse),
        (status = 400, description = "Bad request - Invalid query parameters or validation error"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Search"
)]
pub async fn search_posts(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<SearchPostsRequest>,
) -> Result<SearchPostsResponse, Errors> {
    let response = service_search_posts(&state.meilisearch_client, &payload).await?;
    Ok(response)
}
