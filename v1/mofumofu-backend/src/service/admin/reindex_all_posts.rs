use crate::{
    dto::admin::response::AdminTaskResponse,
    microservices::admin_tasks_client::trigger_reindex_all_posts,
    service::auth::role_check::require_admin,
    service::error::errors::{Errors, ServiceResult},
    state::AppState,
};
use serde_json::json;
use tracing::{error, info};
use uuid::Uuid;

pub async fn service_reindex_all_posts(
    app_state: &AppState,
    user_id: Uuid,
) -> ServiceResult<AdminTaskResponse> {
    // Admin 권한 확인
    require_admin(&app_state.conn, user_id).await?;

    info!("Admin user {} triggering reindex all posts", user_id);

    match trigger_reindex_all_posts(&app_state.http_client).await {
        Ok(response) => {
            info!("Successfully triggered reindex all posts task");
            Ok(AdminTaskResponse {
                success: true,
                message: "전체 포스트 재색인 작업이 시작되었습니다".to_string(),
                data: Some(response),
            })
        }
        Err(e) => {
            error!("Failed to trigger reindex all posts: {}", e);
            Err(Errors::SysInternalError(e.to_string()))
        }
    }
}
