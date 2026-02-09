use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::like::request::check_like_status::CheckLikeStatusRequest;
use crate::dto::like::response::like_status::LikeStatusResponse;
use crate::service::error::errors::Errors;
use crate::service::like::check_post_like_status::service_check_post_like_status;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/like/status",
    request_body = CheckLikeStatusRequest,
    responses(
        (status = 200, description = "Like status retrieved successfully", body = LikeStatusResponse),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Like"
)]
pub async fn check_like_status(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<CheckLikeStatusRequest>,
) -> Result<LikeStatusResponse, Errors> {
    info!("Received request to check like status: {:?}", payload);
    let user_uuid = claims.sub.clone();

    let response =
        service_check_post_like_status(&state.conn, &user_uuid, &payload.post_id).await?;

    Ok(response)
}
