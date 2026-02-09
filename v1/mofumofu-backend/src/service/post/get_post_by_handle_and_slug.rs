use crate::dto::post::response::post_info::{PostAuthor, PostInfoResponse, TocItem};
use crate::microservices::markdown_client::render_markdown;
use crate::repository::hashtag::get_hashtags_by_post::repository_get_hashtags_by_post;
use crate::repository::post::get_post_by_handle_and_slug::repository_get_post_by_handle_and_slug;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use redis::aio::ConnectionManager;
use reqwest::Client;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::{info, warn};

pub async fn service_get_post_by_handle_and_slug<C>(
    conn: &C,
    http_client: &Client,
    handle: &str,
    slug: &str,
) -> ServiceResult<PostInfoResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    let post = repository_get_post_by_handle_and_slug(conn, handle, slug).await?;

    // Get author information
    let user = repository_find_user_by_uuid(conn, &post.user_id)
        .await?
        .ok_or(Errors::UserNotFound)?;

    // Get hashtags for the post
    let hashtags = repository_get_hashtags_by_post(conn, post.id).await?;
    let tags: Vec<String> = hashtags.into_iter().map(|tag| tag.name).collect();

    // 렌더링된 HTML 또는 마크다운 직접 렌더링 사용
    let (rendered_html, toc_items) = match (&post.render, &post.toc) {
        (Some(rendered_html), Some(toc_json)) if !rendered_html.is_empty() => {
            info!(
                "데이터베이스에 저장된 렌더링 결과 사용 (post_id: {}, handle: {}, slug: {})",
                post.id, handle, slug
            );

            // TOC JSON을 파싱
            let toc_items: Vec<TocItem> =
                serde_json::from_value(toc_json.clone()).unwrap_or_else(|e| {
                    warn!("TOC JSON 파싱 실패: {}", e);
                    Vec::new()
                });

            (rendered_html.clone(), toc_items)
        }
        _ => {
            info!(
                "렌더링 결과 없음 - 즉시 렌더링 수행 (post_id: {}, handle: {}, slug: {})",
                post.id, handle, slug
            );

            // 마크다운 직접 렌더링
            match render_markdown(http_client, &post.content).await {
                Ok(rendered_result) => {
                    let toc_items: Vec<TocItem> = rendered_result
                        .toc_items
                        .into_iter()
                        .map(|item| TocItem {
                            level: item.level,
                            text: item.text,
                            id: item.id,
                        })
                        .collect();
                    (rendered_result.html_content, toc_items)
                }
                Err(e) => {
                    warn!("마크다운 렌더링 실패: {}", e);
                    (post.content.clone(), Vec::new()) // 원본 마크다운 반환
                }
            }
        }
    };

    Ok(PostInfoResponse {
        id: post.id,
        title: post.title,
        summary: post.summary,
        thumbnail_image: post.thumbnail_image,
        rendered: rendered_html,
        toc_items,
        author: PostAuthor {
            handle: user.handle,
            name: user.name,
            profile_image: user.profile_image,
        },
        created_at: post.created_at,
        updated_at: post.updated_at,
        like_count: post.like_count,
        comment_count: post.comment_count,
        view_count: post.view_count,
        slug: post.slug,
        tags,
    })
}
