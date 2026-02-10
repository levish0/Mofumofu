use crate::service::posts::service_get_posts;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::posts::{GetPostsRequest, PostListResponse};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/posts",
    params(GetPostsRequest),
    responses(
        (status = 200, description = "Posts retrieved successfully", body = PostListResponse),
        (status = 400, description = "Bad request - Invalid query parameters"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Posts"
)]
pub async fn get_posts(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<GetPostsRequest>,
) -> Result<PostListResponse, Errors> {
    service_get_posts(&state.read_db, payload).await
}
