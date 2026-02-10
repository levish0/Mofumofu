use crate::extractors::RequiredSession;
use crate::service::comments::service_create_comment;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mofumofu_dto::comments::CreateCommentRequest;
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/comments",
    request_body = CreateCommentRequest,
    responses(
        (status = 201, description = "Comment created successfully", body = mofumofu_dto::comments::CommentResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 404, description = "Not Found - Post or parent comment does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Comments"
)]
pub async fn create_comment(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<CreateCommentRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response =
        service_create_comment(&state.write_db, session_context.user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
