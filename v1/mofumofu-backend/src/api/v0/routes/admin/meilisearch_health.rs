use axum::{Extension, extract::State};

use crate::{
    dto::admin::response::AdminTaskResponse, dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::meilisearch_health::service_meilisearch_health, service::error::errors::Errors,
    state::AppState,
};

/// Meilisearch 헬스체크
#[utoipa::path(
    post,
    path = "/v0/admin/search/health",
    summary = "Meilisearch health check",
    description = "Check Meilisearch server health status. (Admin only)",
    responses(
        (status = 200, description = "Health check completed successfully", body = AdminTaskResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn meilisearch_health(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, Errors> {
    let response = service_meilisearch_health(&app_state, token_data.sub).await?;

    Ok(response)
}
