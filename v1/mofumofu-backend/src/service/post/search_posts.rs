use crate::connection::meilisearch::MeilisearchClient;
use crate::dto::post::request::{PostSortOrder, SearchPostsRequest};
use crate::dto::post::response::{GetPostsResponse, PostListItem};
use crate::repository::hashtag::get_hashtags_by_post::repository_get_hashtags_by_posts;
use crate::repository::post::get_posts::repository_get_posts_by_ids;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::service::meilisearch::post_indexer;
use chrono::{DateTime, Utc};
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn service_search_posts<C>(
    conn: &C,
    meilisearch: &MeilisearchClient,
    request: SearchPostsRequest,
) -> ServiceResult<GetPostsResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    let page = request.page.unwrap_or(1);
    let page_size = request.page_size.unwrap_or(20);
    let sort_order = request.sort.unwrap_or(PostSortOrder::Latest);

    // 날짜를 Unix timestamp로 변환
    let date_from = request.date_from.map(|dt| dt.timestamp());
    let date_to = request.date_to.map(|dt| dt.timestamp());

    // 정렬 문자열 변환
    let sort_str = match sort_order {
        PostSortOrder::Popular => "popular",
        PostSortOrder::Oldest => "oldest",
        PostSortOrder::Latest => "latest",
    };

    // Meilisearch에서 post ID 검색
    let (post_ids, total_hits) = post_indexer::search_posts(
        meilisearch,
        request.query.as_deref(),
        request.hashtags.as_deref(),
        request.user_handle.as_deref(),
        date_from,
        date_to,
        request.min_likes,
        sort_str,
        page,
        page_size,
    )
    .await
    .map_err(|e| {
        tracing::warn!("Meilisearch search failed: {}", e);
        Errors::SysInternalError(format!("Search service error: {}", e))
    })?;

    if post_ids.is_empty() {
        return Ok(GetPostsResponse {
            posts: Vec::new(),
            current_page: page,
            page_size,
            has_more: false,
            total_count: total_hits,
        });
    }

    // DB에서 실제 post 데이터 조회
    let posts = repository_get_posts_by_ids(conn, &post_ids).await?;

    // DB 결과를 PostListItem으로 변환 (get_posts.rs와 동일한 로직)
    let post_ids_uuid: Vec<uuid::Uuid> = posts.iter().map(|p| p.id).collect();
    let post_hashtags_map = repository_get_hashtags_by_posts(conn, &post_ids_uuid)
        .await?
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();

    let mut post_items = Vec::new();
    for post in posts {
        let user = repository_find_user_by_uuid(conn, &post.user_id)
            .await?
            .ok_or(Errors::UserNotFound)?;

        let hashtags = post_hashtags_map
            .get(&post.id)
            .map(|tags| tags.iter().map(|tag| tag.name.clone()).collect())
            .unwrap_or_else(Vec::new);

        post_items.push(PostListItem {
            id: post.id,
            title: post.title,
            summary: post.summary,
            thumbnail_image: post.thumbnail_image,
            user_handle: user.handle,
            user_name: user.name,
            user_avatar: user.profile_image,
            created_at: post.created_at,
            like_count: post.like_count,
            comment_count: post.comment_count,
            view_count: post.view_count,
            slug: post.slug,
            hashtags,
        });
    }

    let has_more = post_items.len() == page_size as usize;

    Ok(GetPostsResponse {
        posts: post_items,
        current_page: page,
        page_size,
        has_more,
        total_count: total_hits,
    })
}
