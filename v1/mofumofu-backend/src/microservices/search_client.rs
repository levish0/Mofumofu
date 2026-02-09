use crate::config::db_config::DbConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tracing::{error, info, warn};
use uuid::Uuid;

#[derive(Deserialize)]
struct TaskResponse {
    task_id: String,
    status: String,
    message: String,
}

/// 태스크 서버 URL을 캐시하는 정적 변수
static TASK_SERVER_URL: LazyLock<String> = LazyLock::new(|| {
    let config = DbConfig::get();
    format!(
        "http://{}:{}",
        config.task_server_host, config.task_server_port
    )
});

/// 태스크 서버 URL을 가져오는 함수
fn get_task_server_url() -> &'static str {
    &TASK_SERVER_URL
}

/// 단일 포스트 색인을 태스크 서버에 요청
pub async fn queue_index_post(
    http_client: &Client,
    post_id: &Uuid,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!("Queuing post indexing task for post: {}", post_id);

    let request_body = serde_json::json!({
        "post_id": post_id.to_string()
    });

    let response = http_client
        .post(&format!("{}/tasks/search/index", task_server_url))
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!(
            "Post indexing task queue failed: {} - {}",
            status, error_text
        );
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Post indexing task queued with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}

/// 단일 포스트 색인 업데이트를 태스크 서버에 요청
pub async fn queue_update_post(
    http_client: &Client,
    post_id: &Uuid,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!("Queuing post update task for post: {}", post_id);

    let request_body = serde_json::json!({
        "post_id": post_id.to_string()
    });

    let response = http_client
        .put(&format!("{}/tasks/search/update", task_server_url))
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!("Post update task queue failed: {} - {}", status, error_text);
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!("Post update task queued with ID: {}", task_response.task_id);

    Ok(task_response.task_id)
}

/// 단일 포스트 색인 삭제를 태스크 서버에 요청
pub async fn queue_delete_post(
    http_client: &Client,
    post_id: &Uuid,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!("Queuing post deletion task for post: {}", post_id);

    let request_body = serde_json::json!({
        "post_id": post_id.to_string()
    });

    let response = http_client
        .delete(&format!("{}/tasks/search/delete", task_server_url))
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!(
            "Post deletion task queue failed: {} - {}",
            status, error_text
        );
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Post deletion task queued with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}
