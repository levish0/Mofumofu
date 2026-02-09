use axum::{Extension, extract::State, http::StatusCode};

use crate::{
    dto::admin::response::AdminTaskResponse, dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::cleanup_expired_tokens::service_cleanup_expired_tokens,
    service::error::errors::Errors, state::AppState,
};

/// 만료된 토큰 정리
#[utoipa::path(
    post,
    path = "/v0/admin/cleanup/tokens",
    summary = "Clean up expired refresh tokens",
    description = "Queue task to clean up expired or revoked refresh tokens from database. (Admin only)",
    responses(
        (status = 200, description = "Token cleanup task queued successfully", body = AdminTaskResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn cleanup_expired_tokens(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, Errors> {
    let response = service_cleanup_expired_tokens(&app_state, token_data.sub).await?;

    Ok(response)
}
