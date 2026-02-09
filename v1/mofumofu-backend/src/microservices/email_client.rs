use crate::config::db_config::DbConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tracing::{error, info, warn};

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

/// 일반 이메일 보내기를 태스크 서버에 요청
pub async fn queue_send_email(
    http_client: &Client,
    email_to: &str,
    subject: &str,
    html_content: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!("Queuing email send task to: {}", email_to);

    let request_body = serde_json::json!({
        "email_to": email_to,
        "subject": subject,
        "html_content": html_content
    });

    let response = http_client
        .post(&format!("{}/tasks/email/send", task_server_url))
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!("Email send task queue failed: {} - {}", status, error_text);
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!("Email send task queued with ID: {}", task_response.task_id);

    Ok(task_response.task_id)
}

/// 비밀번호 재설정 이메일 보내기를 태스크 서버에 요청
pub async fn queue_send_reset_password_email(
    http_client: &Client,
    email_to: &str,
    email: &str,
    token: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!("Queuing reset password email task to: {}", email_to);

    let request_body = serde_json::json!({
        "email_to": email_to,
        "email": email,
        "token": token
    });

    let response = http_client
        .post(&format!(
            "{}/tasks/email/send-reset-password",
            task_server_url
        ))
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
            "Reset password email task queue failed: {} - {}",
            status, error_text
        );
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Reset password email task queued with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}

/// 이메일 인증 메일 보내기를 태스크 서버에 요청
pub async fn queue_send_email_verification(
    http_client: &Client,
    email_to: &str,
    username: &str,
    verification_token: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!("Queuing email verification task to: {}", email_to);

    let request_body = serde_json::json!({
        "email_to": email_to,
        "username": username,
        "verification_token": verification_token
    });

    let response = http_client
        .post(&format!(
            "{}/tasks/email/send-verification",
            task_server_url
        ))
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
            "Email verification task queue failed: {} - {}",
            status, error_text
        );
        return Err(format!("Task queue request failed: {} - {}", status, error_text).into());
    }

    let task_response: TaskResponse = response.json().await?;
    info!(
        "Email verification task queued with ID: {}",
        task_response.task_id
    );

    Ok(task_response.task_id)
}

/// 이메일 작업 상태를 태스크 서버에서 확인
pub async fn get_email_task_status(
    http_client: &Client,
    task_id: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
    let task_server_url = get_task_server_url();

    info!("Getting email task status for ID: {}", task_id);

    let response = http_client
        .get(&format!(
            "{}/tasks/email/status/{}",
            task_server_url, task_id
        ))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        warn!(
            "Email task status request failed: {} - {}",
            status, error_text
        );
        return Err(format!("Task status request failed: {} - {}", status, error_text).into());
    }

    let status_response: serde_json::Value = response.json().await?;
    Ok(status_response)
}
