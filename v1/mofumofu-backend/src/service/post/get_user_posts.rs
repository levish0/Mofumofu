use crate::dto::post::response::{PostListItem, UserPostsResponse};
use crate::repository::hashtag::get_hashtags_by_post::repository_get_hashtags_by_posts;
use crate::repository::post::get_user_posts::repository_get_user_posts;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::repository::user::get_user_by_handle::repository_get_user_by_handle;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn service_get_user_posts<C>(
    conn: &C,
    user_handle: &str,
) -> ServiceResult<UserPostsResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    // 사용자 조회
    let user = repository_get_user_by_handle(conn, user_handle).await?;

    // 해당 사용자의 모든 글 조회
    let posts = repository_get_user_posts(conn, user.id).await?;

    if posts.is_empty() {
        return Ok(UserPostsResponse { posts: Vec::new() });
    }

    // DB 결과를 PostListItem으로 변환
    let post_ids: Vec<uuid::Uuid> = posts.iter().map(|p| p.id).collect();
    let post_hashtags_map = repository_get_hashtags_by_posts(conn, &post_ids)
        .await?
        .into_iter()
        .collect::<std::collections::HashMap<_, _>>();

    let mut post_items = Vec::new();
    for post in &posts {
        let post_user = repository_find_user_by_uuid(conn, &post.user_id)
            .await?
            .ok_or(Errors::UserNotFound)?;

        let hashtags = post_hashtags_map
            .get(&post.id)
            .map(|tags| tags.iter().map(|tag| tag.name.clone()).collect())
            .unwrap_or_else(Vec::new);

        post_items.push(PostListItem {
            id: post.id,
            title: post.title.clone(),
            summary: post.summary.clone(),
            thumbnail_image: post.thumbnail_image.clone(),
            user_handle: post_user.handle,
            user_name: post_user.name,
            user_avatar: post_user.profile_image,
            created_at: post.created_at,
            like_count: post.like_count,
            comment_count: post.comment_count,
            view_count: post.view_count,
            slug: post.slug.clone(),
            hashtags,
        });
    }

    Ok(UserPostsResponse { posts: post_items })
}
