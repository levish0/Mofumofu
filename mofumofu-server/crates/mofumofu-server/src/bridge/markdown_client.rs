use mofumofu_config::ServerConfig;
use mofumofu_errors::errors::Errors;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tracing::error;

#[derive(Serialize)]
struct RenderRequest<'a> {
    markdown: &'a str,
}

#[derive(Deserialize)]
struct RenderResponse {
    success: bool,
    data: Option<RenderData>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RenderData {
    html_content: String,
    toc_items: JsonValue,
}

/// Render markdown via the markdown service.
///
/// Returns `(html, toc_items)` on success.
pub async fn render_markdown(
    http_client: &HttpClient,
    content: &str,
) -> Result<(String, JsonValue), Errors> {
    let config = ServerConfig::get();
    let url = format!("{}/render", config.markdown_service_url);

    let response = http_client
        .post(&url)
        .json(&RenderRequest { markdown: content })
        .send()
        .await
        .map_err(|e| {
            error!("Markdown service request failed: {}", e);
            Errors::MarkdownRenderFailed
        })?;

    if !response.status().is_success() {
        error!(
            "Markdown service returned error status: {}",
            response.status()
        );
        return Err(Errors::MarkdownRenderFailed);
    }

    let render_response: RenderResponse = response.json().await.map_err(|e| {
        error!("Failed to parse markdown service response: {}", e);
        Errors::MarkdownRenderFailed
    })?;

    if !render_response.success {
        error!("Markdown service returned success=false");
        return Err(Errors::MarkdownRenderFailed);
    }

    let data = render_response.data.ok_or_else(|| {
        error!("Markdown service returned no data");
        Errors::MarkdownRenderFailed
    })?;

    Ok((data.html_content, data.toc_items))
}
