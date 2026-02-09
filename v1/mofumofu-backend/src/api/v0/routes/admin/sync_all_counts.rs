use axum::{Extension, extract::State};

use crate::{
    dto::admin::response::AdminTaskResponse, dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::sync_all_counts::service_sync_all_counts, service::error::errors::Errors,
    state::AppState,
};

/// 전체 카운트 동기화
#[utoipa::path(
    post,
    path = "/v0/admin/sync/all",
    summary = "Sync all counts",
    description = "Synchronize all counts (likes, follows) with actual data. (Admin only)",
    responses(
        (status = 200, description = "Sync task started successfully", body = AdminTaskResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn sync_all_counts(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, Errors> {
    let response = service_sync_all_counts(&app_state, token_data.sub).await?;

    Ok(response)
}
