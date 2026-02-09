use axum::{Extension, extract::State};

use crate::{
    dto::admin::response::AdminTaskResponse, dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::sync_likes::service_sync_likes, service::error::errors::Errors,
    state::AppState,
};

/// 포스트 좋아요 수 동기화
#[utoipa::path(
    post,
    path = "/v0/admin/sync/likes",
    summary = "Sync post like counts",
    description = "Synchronize all posts' like counts with actual data. (Admin only)",
    responses(
        (status = 200, description = "Like sync task started successfully", body = AdminTaskResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Admin access required"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn sync_likes(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminTaskResponse, Errors> {
    let response = service_sync_likes(&app_state, token_data.sub).await?;

    Ok(response)
}
