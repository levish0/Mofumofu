use crate::{
    dto::admin::response::AdminTaskResponse,
    microservices::admin_tasks_client::check_meilisearch_health,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_meilisearch_health(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<AdminTaskResponse> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} checking Meilisearch health", user_id);

    match check_meilisearch_health(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully checked Meilisearch health");
            Ok(AdminTaskResponse {
                success: true,
                message: "Meilisearch 헬스 체크를 성공적으로 완료했습니다".to_string(),
                data: Some(response),
            })
        }
        Err(e) => {
            error!("Failed to check Meilisearch health: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}
