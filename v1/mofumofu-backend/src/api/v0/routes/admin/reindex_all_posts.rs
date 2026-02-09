use axum::{Extension, extract::State};

use crate::{
    dto::admin::response::AdminTaskResponse, dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::reindex_all_posts::service_reindex_all_posts, service::error::errors::Errors,
    state::AppState,
};

/// 전체 포스트 재색인 트리거
#[utoipa::path(
    post,
    path = "/v0/admin/search/reindex-all",
    summary = "Trigger full posts reindex",
    description = "Reindex all posts in Meilisearch. (Admin only)",
    responses(
        (status = 200, description = "Reindex task started successfully", body = AdminTaskResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn reindex_all_posts(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, Errors> {
    let response = service_reindex_all_posts(&app_state, token_data.sub).await?;

    Ok(response)
}
