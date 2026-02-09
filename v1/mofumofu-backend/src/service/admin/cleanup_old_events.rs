use crate::{
    dto::admin::response::AdminTaskResponse,
    microservices::admin_tasks_client::cleanup_old_system_events,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_cleanup_old_events(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<AdminTaskResponse> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} triggering old events cleanup", user_id);

    match cleanup_old_system_events(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully triggered old events cleanup");
            Ok(AdminTaskResponse {
                success: true,
                message: "오래된 시스템 이벤트 정리 작업이 시작되었습니다".to_string(),
                data: Some(response),
            })
        }
        Err(e) => {
            error!("Failed to cleanup old events: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}
