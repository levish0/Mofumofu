use crate::service::comments::service_get_comments;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::comments::{CommentListResponse, GetCommentsRequest};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/comments",
    params(GetCommentsRequest),
    responses(
        (status = 200, description = "Comments retrieved successfully", body = CommentListResponse),
        (status = 400, description = "Bad request - Invalid query parameters"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Comments"
)]
pub async fn get_comments(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<GetCommentsRequest>,
) -> Result<CommentListResponse, Errors> {
    service_get_comments(&state.read_db, payload).await
}
