use crate::extractors::RequiredSession;
use crate::service::comments::service_update_comment;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use mofumofu_dto::comments::{CommentIdPath, CommentResponse, UpdateCommentRequest};
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    patch,
    path = "/v0/comments/{comment_id}",
    params(CommentIdPath),
    request_body = UpdateCommentRequest,
    responses(
        (status = 200, description = "Comment updated successfully", body = CommentResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
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
pub async fn update_comment(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<CommentIdPath>,
    ValidatedJson(payload): ValidatedJson<UpdateCommentRequest>,
) -> Result<Json<CommentResponse>, Errors> {
    let response = service_update_comment(
        &state.write_db,
        session_context.user_id,
        path.comment_id,
        payload,
    )
    .await?;
    Ok(Json(response))
}
