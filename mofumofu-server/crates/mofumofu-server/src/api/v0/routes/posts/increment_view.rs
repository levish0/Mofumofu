use crate::service::posts::service_increment_post_view;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use mofumofu_dto::posts::PostIdPath;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/posts/{post_id}/view",
    params(PostIdPath),
    responses(
        (status = 204, description = "View count incremented"),
        (status = 400, description = "Bad request - Invalid path parameter"),
        (status = 404, description = "Not Found - Post does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Posts"
)]
pub async fn increment_view(
    State(state): State<AppState>,
    ValidatedPath(path): ValidatedPath<PostIdPath>,
) -> Result<StatusCode, Errors> {
    service_increment_post_view(&state.write_db, path.post_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
