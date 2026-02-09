use crate::dto::post::request::SearchPostsRequest;
use crate::dto::post::response::GetPostsResponse;
use crate::service::error::errors::Errors;
use crate::service::post::search_posts::service_search_posts;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/posts/search",
    request_body = SearchPostsRequest,
    responses(
        (status = 200, description = "Posts search completed successfully", body = GetPostsResponse),
        (status = 400, description = "Invalid search parameters"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error"),
        (status = 503, description = "Search service unavailable")
    ),
    tag = "Post"
)]
pub async fn search_posts(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<SearchPostsRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received POST request to search posts: {:?}", payload);

    let response = service_search_posts(&state.conn, &state.meilisearch, payload).await?;

    Ok(response)
}
