use crate::extractors::RequiredSession;
use crate::service::comments::service_delete_comment;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use mofumofu_dto::comments::CommentIdPath;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    delete,
    path = "/v0/comments/{comment_id}",
    params(CommentIdPath),
    responses(
        (status = 204, description = "Comment deleted successfully"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 403, description = "Forbidden - Not the comment author"),
        (status = 404, description = "Not Found - Comment does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Comments"
)]
pub async fn delete_comment(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<CommentIdPath>,
) -> Result<StatusCode, Errors> {
    service_delete_comment(&state.write_db, session_context.user_id, path.comment_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
