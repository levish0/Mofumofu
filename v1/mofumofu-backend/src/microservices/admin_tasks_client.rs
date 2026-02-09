use crate::config::db_config::DbConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tracing::{error, info, warn};

#[derive(Deserialize, Clone)]
pub struct TaskResponse {
    pub task_id: String,
    pub status: String,
    pub message: String,
}

#[derive(Deserialize, Clone)]
pub struct CountSyncResponse {
    pub status: String,
    pub updated_count: Option<u64>,
    pub follower_updated_count: Option<u64>,
    pub following_updated_count: Option<u64>,
    pub total_updated: Option<u64>,
    pub message: String,
}

#[derive(Deserialize, Clone)]
pub struct TokenCleanupResponse {
    pub status: String,
    pub expired_tokens_deleted: Option<u64>,
    pub revoked_tokens_deleted: Option<u64>,
    pub total_deleted: Option<u64>,
    pub deleted_count: Option<u64>,
    pub message: String,
}

#[derive(Deserialize, Clone)]
pub struct SearchStatsResponse {
    pub status: String,
    pub stats: Option<serde_json::Value>,
    pub health: Option<serde_json::Value>,
    pub indexed_count: Option<u64>,
    pub message: String,
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

// === Search Tasks ===

/// 전체 포스트 재색인 작업 트리거
pub async fn trigger_reindex_all_posts(
    http_client: &Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();
    info!("Triggering reindex all posts task");

    let response = http_client
        .post(&format!("{}/tasks/search-reindex/reindex", task_server_url))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!("Reindex all posts task failed: {} - {}", status, error_text);
        return Err(format!("Task request failed: {} - {}", status, error_text).into());
    }

    let task_response: serde_json::Value = response.json().await?;
    info!("Reindex all posts task triggered successfully");
    Ok(task_response)
}

/// Meilisearch 헬스체크 실행
pub async fn check_meilisearch_health(
    http_client: &Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();
    info!("Checking Meilisearch health");

    let response = http_client
        .get(&format!("{}/tasks/search-reindex/health", task_server_url))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!(
            "Meilisearch health check failed: {} - {}",
            status, error_text
        );
        return Err(format!("Health check failed: {} - {}", status, error_text).into());
    }

    let task_response: serde_json::Value = response.json().await?;
    info!("Meilisearch health check completed");
    Ok(task_response)
}

/// 검색 색인 통계 조회
pub async fn get_search_stats(
    http_client: &Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();
    info!("Getting search index stats");

    let response = http_client
        .get(&format!("{}/tasks/search-reindex/stats", task_server_url))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!("Get search stats failed: {} - {}", status, error_text);
        return Err(format!("Get stats failed: {} - {}", status, error_text).into());
    }

    let task_response: serde_json::Value = response.json().await?;
    info!("Search stats retrieved successfully");
    Ok(task_response)
}

// === Count Sync Tasks ===

/// 포스트 좋아요 수 동기화 트리거
pub async fn sync_post_like_counts(
    http_client: &Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();
    info!("Triggering post like counts sync");

    let response = http_client
        .post(&format!("{}/tasks/count/sync/likes", task_server_url))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!("Post like counts sync failed: {} - {}", status, error_text);
        return Err(format!("Sync failed: {} - {}", status, error_text).into());
    }

    let task_response: serde_json::Value = response.json().await?;
    info!("Post like counts sync triggered successfully");
    Ok(task_response)
}

/// 유저 팔로우 수 동기화 트리거
pub async fn sync_user_follow_counts(
    http_client: &Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();
    info!("Triggering user follow counts sync");

    let response = http_client
        .post(&format!("{}/tasks/count/sync/follows", task_server_url))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!(
            "User follow counts sync failed: {} - {}",
            status, error_text
        );
        return Err(format!("Sync failed: {} - {}", status, error_text).into());
    }

    let task_response: serde_json::Value = response.json().await?;
    info!("User follow counts sync triggered successfully");
    Ok(task_response)
}

/// 전체 카운트 동기화 트리거
pub async fn sync_all_counts(
    http_client: &Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();
    info!("Triggering all counts sync");

    let response = http_client
        .post(&format!("{}/tasks/count/sync/all", task_server_url))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!("All counts sync failed: {} - {}", status, error_text);
        return Err(format!("Sync failed: {} - {}", status, error_text).into());
    }

    let task_response: serde_json::Value = response.json().await?;
    info!("All counts sync triggered successfully");
    Ok(task_response)
}

// === Token Cleanup Tasks ===

/// 만료된 리프레시 토큰 정리 트리거
pub async fn cleanup_expired_refresh_tokens(
    http_client: &Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();
    info!("Triggering expired refresh tokens cleanup");

    let response = http_client
        .post(&format!("{}/tasks/token-cleanup/cleanup", task_server_url))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!("Refresh tokens cleanup failed: {} - {}", status, error_text);
        return Err(format!("Cleanup failed: {} - {}", status, error_text).into());
    }

    let task_response: serde_json::Value = response.json().await?;
    info!("Expired refresh tokens cleanup triggered successfully");
    Ok(task_response)
}

/// 오래된 시스템 이벤트 정리 트리거
pub async fn cleanup_old_system_events(
    http_client: &Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();
    info!("Triggering old system events cleanup");

    let response = http_client
        .post(&format!(
            "{}/tasks/token-cleanup/cleanup-events",
            task_server_url
        ))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!("System events cleanup failed: {} - {}", status, error_text);
        return Err(format!("Cleanup failed: {} - {}", status, error_text).into());
    }

    let task_response: serde_json::Value = response.json().await?;
    info!("Old system events cleanup triggered successfully");
    Ok(task_response)
}
