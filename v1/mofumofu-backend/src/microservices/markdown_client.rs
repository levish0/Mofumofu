use crate::config::db_config::DbConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Serialize)]
struct MarkdownRequest {
    markdown: String,
}

#[derive(Serialize)]
struct MarkdownRenderRequest {
    post_id: String,
    content: String,
}

#[derive(Deserialize)]
struct TocItem {
    level: i32,
    text: String,
    id: String,
}

#[derive(Deserialize)]
struct MarkdownData {
    #[serde(rename = "htmlContent")]
    html_content: String,
    #[serde(rename = "tocItems")]
    toc_items: Vec<TocItem>,
}

#[derive(Deserialize)]
struct MarkdownResponse {
    success: bool,
    data: Option<MarkdownData>,
    error: Option<String>,
}

#[derive(Debug)]
pub struct RenderedMarkdown {
    pub html_content: String,
    pub toc_items: Vec<TableOfContentsItem>,
}

#[derive(Debug)]
pub struct TableOfContentsItem {
    pub level: i32,
    pub text: String,
    pub id: String,
}

/// 마크다운 서비스 URL을 캐시하는 정적 변수
static MARKDOWN_SERVICE_URL: LazyLock<String> = LazyLock::new(|| {
    let config = DbConfig::get();
    format!(
        "http://{}:{}",
        config.markdown_service_host, config.markdown_service_port
    )
});

/// 태스크 서비스 URL을 캐시하는 정적 변수
static TASKS_SERVICE_URL: LazyLock<String> = LazyLock::new(|| {
    let config = DbConfig::get();
    format!(
        "http://{}:{}",
        config.task_server_host, config.task_server_port
    )
});

/// 마크다운 서비스 URL을 가져오는 함수
fn get_markdown_service_url() -> &'static str {
    &MARKDOWN_SERVICE_URL
}

/// 태스크 서비스 URL을 가져오는 함수
fn get_tasks_service_url() -> &'static str {
    &TASKS_SERVICE_URL
}

/// 마크다운을 HTML로 렌더링
pub async fn render_markdown(
    http_client: &Client,
    markdown: &str,
) -> Result<RenderedMarkdown, Box<dyn std::error::Error + Send + Sync>> {
    let service_url = get_markdown_service_url();

    info!("Rendering markdown, length: {} chars", markdown.len());

    let request = MarkdownRequest {
        markdown: markdown.to_string(),
    };

    let response = http_client
        .post(&format!("{}/render", service_url))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        error!(
            "Markdown service request failed: {} - {}",
            status, error_text
        );
        return Err(format!(
            "Markdown service request failed: {} - {}",
            status, error_text
        )
        .into());
    }

    let markdown_response: MarkdownResponse = response.json().await?;

    if !markdown_response.success {
        let error_msg = markdown_response
            .error
            .unwrap_or_else(|| "Unknown error".to_string());
        error!("Markdown processing failed: {}", error_msg);
        return Err(format!("Markdown processing failed: {}", error_msg).into());
    }

    let data = markdown_response.data.ok_or("Missing data in response")?;

    let toc_items: Vec<TableOfContentsItem> = data
        .toc_items
        .into_iter()
        .map(|item| TableOfContentsItem {
            level: item.level,
            text: item.text,
            id: item.id,
        })
        .collect();

    info!(
        "Markdown rendered successfully, TOC items: {}",
        toc_items.len()
    );

    Ok(RenderedMarkdown {
        html_content: data.html_content,
        toc_items,
    })
}

/// 마크다운 렌더링 태스크를 큐에 추가
pub async fn queue_render_markdown(
    http_client: &Client,
    post_id: &Uuid,
    content: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let service_url = get_tasks_service_url();

    info!("마크다운 렌더링 태스크 큐에 추가: post_id={}", post_id);

    let request = MarkdownRenderRequest {
        post_id: post_id.to_string(),
        content: content.to_string(),
    };

    let response = http_client
        .post(&format!("{}/tasks/markdown/render", service_url))
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        error!(
            "마크다운 렌더링 태스크 큐 추가 실패: {} - {}",
            status, error_text
        );
        return Err(format!(
            "마크다운 렌더링 태스크 큐 추가 실패: {} - {}",
            status, error_text
        )
        .into());
    }

    info!("마크다운 렌더링 태스크 큐에 추가 완료: post_id={}", post_id);
    Ok(())
}
