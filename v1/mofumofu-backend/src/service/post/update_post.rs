use crate::dto::post::request::update_post::UpdatePostRequest;
use crate::entity::common::{ActionType, TargetType};
use crate::microservices::markdown_client::render_markdown;
use crate::microservices::search_client;
use crate::repository::hashtag::associate_post_hashtags::repository_associate_post_hashtags;
use crate::repository::hashtag::remove_post_hashtags::repository_remove_post_hashtags;
use crate::repository::post::update_post::repository_update_post;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use serde_json::json;
use tracing::{info, warn};
use uuid::Uuid;

pub async fn service_update_post<C>(
    conn: &C,
    http_client: &reqwest::Client,
    payload: UpdatePostRequest,
    user_uuid: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    // 마크다운 렌더링 (content가 변경되는 경우에만)
    let (render_html, toc_json) = if let Some(ref content) = payload.content {
        info!("마크다운 렌더링 시작 (content length: {})", content.len());
        let rendered = render_markdown(http_client, content)
            .await
            .map_err(|e| Errors::MarkdownRenderFailed(format!("마크다운 렌더링 실패: {}", e)))?;

        info!(
            "마크다운 렌더링 성공 (TOC items: {})",
            rendered.toc_items.len()
        );

        // TOC를 JSON으로 변환
        let toc_items: Vec<serde_json::Value> = rendered
            .toc_items
            .into_iter()
            .map(|item| {
                json!({
                    "level": item.level,
                    "text": item.text,
                    "id": item.id
                })
            })
            .collect();

        (Some(rendered.html_content), Some(json!(toc_items)))
    } else {
        (None, None)
    };

    let txn = conn.begin().await?;

    let updated_post =
        repository_update_post(&txn, payload.clone(), user_uuid, render_html, toc_json).await?;

    if let Some(ref hashtags) = payload.hashtags {
        repository_remove_post_hashtags(&txn, updated_post.id).await?;

        if !hashtags.is_empty() {
            repository_associate_post_hashtags(&txn, updated_post.id, hashtags, *user_uuid).await?;
        }
    }

    txn.commit().await?;

    if let Err(e) = search_client::queue_update_post(http_client, &updated_post.id).await {
        warn!("Failed to queue post search update task: {}", e);
    }

    info!("글 수정 완료 (post_id: {})", updated_post.id);

    repository_log_event(
        conn,
        Some(*user_uuid),
        ActionType::PostUpdated,
        Some(updated_post.id),
        Some(TargetType::Post),
        None,
    )
    .await;

    Ok(())
}
