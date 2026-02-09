use crate::dto::post::response::post_info::{PostAuthor, PostInfoResponse};
use crate::repository::hashtag::get_hashtags_by_post::repository_get_hashtags_by_post;
use crate::repository::post::get_post_by_uuid::repository_get_post_by_uuid;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use reqwest::Client;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::info;
use uuid::Uuid;

pub async fn service_get_post_by_uuid<C>(
    conn: &C,
    _http_client: &Client,
    post_id: &Uuid,
) -> ServiceResult<PostInfoResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    info!("Getting post by UUID: {}", post_id);

    // 포스트 조회
    let post = repository_get_post_by_uuid(conn, post_id).await?;

    // 작성자 정보 조회
    let user = repository_find_user_by_uuid(conn, &post.user_id)
        .await?
        .ok_or(Errors::UserNotFound)?;

    // 해시태그 조회
    let hashtags = repository_get_hashtags_by_post(conn, *post_id)
        .await?
        .into_iter()
        .map(|hashtag| hashtag.name)
        .collect();

    Ok(PostInfoResponse {
        id: post.id,
        title: post.title,
        summary: post.summary,
        thumbnail_image: post.thumbnail_image,
        rendered: post.render.unwrap_or_default(), // 렌더링된 HTML 또는 빈 문자열
        toc_items: Vec::new(),                     // TODO: TOC 파싱 구현 필요
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
        tags: hashtags,
    })
}
