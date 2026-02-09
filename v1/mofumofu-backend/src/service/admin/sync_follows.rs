use crate::{
    dto::admin::response::AdminTaskResponse,
    microservices::admin_tasks_client::sync_user_follow_counts,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_sync_follows(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<AdminTaskResponse> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} triggering follow counts sync", user_id);

    match sync_user_follow_counts(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully triggered follow counts sync");
            Ok(AdminTaskResponse {
                success: true,
                message: "팔로우 수 동기화 작업이 시작되었습니다".to_string(),
                data: Some(response),
            })
        }
        Err(e) => {
            error!("Failed to sync follow counts: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}
