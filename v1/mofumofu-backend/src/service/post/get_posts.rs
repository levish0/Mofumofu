use crate::dto::post::request::{GetPostsRequest, PostSortOrder};
use crate::dto::post::response::{GetPostsResponse, PostListItem};
use crate::repository::hashtag::get_hashtags_by_post::repository_get_hashtags_by_posts;
use crate::repository::post::get_posts::{repository_get_posts, repository_get_posts_count};
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn service_get_posts<C>(
    conn: &C,
    request: GetPostsRequest,
) -> ServiceResult<GetPostsResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    let page = request.page.unwrap_or(1);
    let page_size = request.page_size.unwrap_or(8);
    let sort_order = request.sort.unwrap_or(PostSortOrder::Latest);

    // DB에서 직접 포스트 조회
    let posts = repository_get_posts(conn, page, page_size, &sort_order).await?;

    if posts.is_empty() {
        return Ok(GetPostsResponse {
            posts: Vec::new(),
            current_page: page,
            page_size,
            has_more: false,
            total_count: 0,
        });
    }

    // DB 결과를 PostListItem으로 변환
    let post_ids: Vec<uuid::Uuid> = posts.iter().map(|p| p.id).collect();
    let post_hashtags_map = repository_get_hashtags_by_posts(conn, &post_ids)
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
    let total_count = repository_get_posts_count(conn).await?;

    Ok(GetPostsResponse {
        posts: post_items,
        current_page: page,
        page_size,
        has_more,
        total_count,
    })
}
