use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::comment::request::delete_comment::DeleteCommentRequest;
use crate::service::comment::delete_comment::service_delete_comment;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    delete,
    path = "/v0/comment",
    request_body = DeleteCommentRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "Comment deleted successfully"),
        (status = StatusCode::NOT_FOUND, description = "Comment not found"),
        (status = StatusCode::UNAUTHORIZED, description = "Not authorized to delete this comment"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comment"
)]
pub async fn delete_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<DeleteCommentRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received request to delete comment: {:?}", payload);
    let user_uuid = claims.sub.clone();

    service_delete_comment(&state.conn, &user_uuid, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
