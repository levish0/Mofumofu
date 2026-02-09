use axum::{Extension, extract::State};

use crate::{
    dto::admin::response::AdminTaskResponse, dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::search_stats::service_search_stats, service::error::errors::Errors,
    state::AppState,
};

/// 검색 색인 통계 조회
#[utoipa::path(
    post,
    path = "/v0/admin/search/stats",
    summary = "Get search index statistics",
    description = "Retrieve Meilisearch index statistics. (Admin only)",
    responses(
        (status = 200, description = "Statistics retrieved successfully", body = AdminTaskResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn search_stats(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, Errors> {
    let response = service_search_stats(&app_state, token_data.sub).await?;

    Ok(response)
}
