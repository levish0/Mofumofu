use crate::dto::post::request::create_post::CreatePostRequest;
use crate::dto::post::response::create_post::CreatePostResponse;
use crate::entity::common::{ActionType, TargetType};
use crate::microservices::markdown_client::render_markdown;
use crate::microservices::search_client;
use crate::repository::hashtag::associate_post_hashtags::repository_associate_post_hashtags;
use crate::repository::post::create_post::repository_create_post;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use serde_json::json;
use tracing::{info, warn};
use uuid::Uuid;

pub async fn service_create_post<C>(
    conn: &C,
    http_client: &reqwest::Client,
    payload: CreatePostRequest,
    user_uuid: &Uuid,
) -> ServiceResult<CreatePostResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    let hashtags = payload.hashtags.clone();
    let content = payload.content.clone();

    // 마크다운 렌더링 (필수)
    info!("마크다운 렌더링 시작 (content length: {})", content.len());
    let rendered = render_markdown(http_client, &content)
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

    let (render_html, toc_json) = (Some(rendered.html_content), Some(json!(toc_items)));

    let txn = conn.begin().await?;

    let post = CreatePostRequest {
        title: payload.title,
        summary: payload.summary,
        content: payload.content,
        slug: payload.slug,
        hashtags: payload.hashtags,
    };

    let created_post = repository_create_post(&txn, post, user_uuid, render_html, toc_json).await?;

    let hashtag_ids = if let Some(ref tags) = hashtags {
        if !tags.is_empty() {
            repository_associate_post_hashtags(&txn, created_post.id, tags, *user_uuid).await?
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    // Commit the transaction
    txn.commit().await?;

    // Python 태스크로 색인 요청 (DB 트랜잭션 외부에서 실행)
    if let Err(e) = search_client::queue_index_post(http_client, &created_post.id).await {
        warn!("Failed to queue post indexing task: {}", e);
    }

    info!("글 생성 완료 (post_id: {})", created_post.id);

    // 이벤트 로깅 - 포스트 생성
    repository_log_event(
        conn,
        Some(*user_uuid),
        ActionType::PostCreated,
        Some(created_post.id),
        Some(TargetType::Post),
        None,
    )
    .await;

    // 해시태그 사용 이벤트 로깅
    for hashtag_id in hashtag_ids {
        repository_log_event(
            conn,
            Some(*user_uuid),
            ActionType::HashtagUsed,
            Some(hashtag_id),
            Some(TargetType::Hashtag),
            None,
        )
        .await;
    }

    Ok(CreatePostResponse {
        post_id: created_post.id,
    })
}
