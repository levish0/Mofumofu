use crate::service::posts::service_get_post_feed;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::posts::{GetPostFeedRequest, PostFeedResponse};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/posts/feed",
    params(GetPostFeedRequest),
    responses(
        (status = 200, description = "Post feed retrieved successfully", body = PostFeedResponse),
        (status = 400, description = "Bad request - Invalid query parameters"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Posts"
)]
pub async fn get_post_feed(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<GetPostFeedRequest>,
) -> Result<PostFeedResponse, Errors> {
    service_get_post_feed(&state.read_db, payload).await
}
