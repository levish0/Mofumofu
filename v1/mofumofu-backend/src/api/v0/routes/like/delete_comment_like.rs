use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::like::request::delete_comment_like::DeleteCommentLikeRequest;
use crate::service::error::errors::Errors;
use crate::service::like::delete_comment_like::service_delete_comment_like;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    delete,
    path = "/v0/comment/like",
    request_body = DeleteCommentLikeRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "Comment unliked successfully"),
        (status = StatusCode::NOT_FOUND, description = "Comment not found or Like not found: like:not_found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Like"
)]
pub async fn delete_comment_like(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<DeleteCommentLikeRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received request to unlike comment: {:?}", payload);
    let user_uuid = claims.sub.clone();

    service_delete_comment_like(&state.conn, &user_uuid, &payload.comment_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
