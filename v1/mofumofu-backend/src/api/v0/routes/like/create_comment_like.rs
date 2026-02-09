use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::like::request::create_comment_like::CreateCommentLikeRequest;
use crate::service::error::errors::Errors;
use crate::service::like::create_comment_like::service_create_comment_like;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/comment/like",
    request_body = CreateCommentLikeRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "Comment liked successfully"),
        (status = StatusCode::NOT_FOUND, description = "Comment not found"),
        (status = StatusCode::CONFLICT, description = "Already liked: like:already_exists"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Like"
)]
pub async fn create_comment_like(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<CreateCommentLikeRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received request to like comment: {:?}", payload);
    let user_uuid = claims.sub.clone();

    service_create_comment_like(&state.conn, &user_uuid, &payload.comment_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
