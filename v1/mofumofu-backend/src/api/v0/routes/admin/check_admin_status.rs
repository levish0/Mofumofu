use axum::{Extension, extract::State};

use crate::{
    dto::admin::response::admin_status::AdminStatusResponse,
    dto::auth::internal::access_token::AccessTokenClaims,
    service::admin::check_admin_status::service_check_admin_status, service::error::errors::Errors,
    state::AppState,
};

/// Admin 권한 확인
#[utoipa::path(
    get,
    path = "/v0/admin/status",
    summary = "Check admin status",
    description = "Check if the current user has admin privileges.",
    responses(
        (status = 200, description = "Admin status checked successfully", body = AdminStatusResponse),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Admin"
)]
pub async fn check_admin_status(
    State(app_state): State<AppState>,
    Extension(token_data): Extension<AccessTokenClaims>,
) -> Result<AdminStatusResponse, Errors> {
    let is_admin = service_check_admin_status(&app_state.conn, token_data.sub).await?;

    Ok(AdminStatusResponse { is_admin })
}
