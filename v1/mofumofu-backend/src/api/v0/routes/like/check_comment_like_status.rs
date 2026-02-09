use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::like::request::check_comment_like_status::CheckCommentLikeStatusRequest;
use crate::dto::like::response::like_status::LikeStatusResponse;
use crate::service::error::errors::Errors;
use crate::service::like::check_comment_like_status::service_check_comment_like_status;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/comment/like/status",
    request_body = CheckCommentLikeStatusRequest,
    responses(
        (status = 200, description = "Comment like status retrieved successfully", body = LikeStatusResponse),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Like"
)]
pub async fn check_comment_like_status(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<CheckCommentLikeStatusRequest>,
) -> Result<LikeStatusResponse, Errors> {
    info!(
        "Received request to check comment like status: {:?}",
        payload
    );
    let user_uuid = claims.sub.clone();

    let response =
        service_check_comment_like_status(&state.conn, &user_uuid, &payload.comment_id).await?;

    Ok(response)
}
