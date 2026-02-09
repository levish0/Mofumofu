use crate::{
    dto::admin::response::AdminTaskResponse,
    microservices::admin_tasks_client::cleanup_expired_refresh_tokens,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_cleanup_expired_tokens(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<AdminTaskResponse> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} triggering expired tokens cleanup", user_id);

    match cleanup_expired_refresh_tokens(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully triggered expired tokens cleanup");
            Ok(AdminTaskResponse {
                success: true,
                message: "만료된 토큰 정리 작업이 시작되었습니다".to_string(),
                data: Some(response),
            })
        }
        Err(e) => {
            error!("Failed to cleanup expired tokens: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}
