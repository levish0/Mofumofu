use crate::{
    dto::admin::response::AdminTaskResponse,
    microservices::admin_tasks_client::get_search_stats,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_search_stats(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<AdminTaskResponse> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} getting search stats", user_id);

    match get_search_stats(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully retrieved search stats");
            Ok(AdminTaskResponse {
                success: true,
                message: "검색 통계를 성공적으로 조회했습니다".to_string(),
                data: Some(response),
            })
        }
        Err(e) => {
            error!("Failed to get search stats: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}
