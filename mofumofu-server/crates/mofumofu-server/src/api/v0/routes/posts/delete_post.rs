use crate::extractors::RequiredSession;
use crate::service::posts::service_delete_post;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use mofumofu_dto::posts::PostIdPath;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    delete,
    path = "/v0/posts/{post_id}",
    params(PostIdPath),
    responses(
        (status = 204, description = "Post deleted successfully"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 403, description = "Forbidden - Not the post author"),
        (status = 404, description = "Not Found - Post does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Posts"
)]
pub async fn delete_post(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<PostIdPath>,
) -> Result<StatusCode, Errors> {
    service_delete_post(
        &state.write_db,
        &state.worker,
        session_context.user_id,
        path.post_id,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}
