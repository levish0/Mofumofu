use axum::{Extension, extract::State, http::StatusCode};

use crate::{
    dto::admin::response::AdminTaskResponse, dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::cleanup_old_events::service_cleanup_old_events, service::error::errors::Errors,
    state::AppState,
};

/// 오래된 시스템 이벤트 정리
#[utoipa::path(
    post,
    path = "/v0/admin/cleanup/events",
    summary = "Clean up old system events",
    description = "Queue task to clean up system events older than 30 days from database. (Admin only)",
    responses(
        (status = 200, description = "Event cleanup task queued successfully", body = AdminTaskResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn cleanup_old_events(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, Errors> {
    let response = service_cleanup_old_events(&app_state, token_data.sub).await?;

    Ok(response)
}
