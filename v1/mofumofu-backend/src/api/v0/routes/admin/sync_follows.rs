use axum::{Extension, extract::State};

use crate::{
    dto::admin::response::AdminTaskResponse, dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::sync_follows::service_sync_follows, service::error::errors::Errors,
    state::AppState,
};

/// 유저 팔로우 수 동기화
#[utoipa::path(
    post,
    path = "/v0/admin/sync/follows",
    summary = "Sync user follow counts",
    description = "Synchronize all users' follower/following counts with actual data. (Admin only)",
    responses(
        (status = 200, description = "Follow sync task started successfully", body = AdminTaskResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn sync_follows(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, Errors> {
    let response = service_sync_follows(&app_state, token_data.sub).await?;

    Ok(response)
}
