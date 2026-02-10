use std::collections::HashMap;

use crate::repository::post_hashtags::repository_find_post_hashtags_by_post_id;
use crate::repository::posts::{
    PostFeedFilter, repository_count_post_feed, repository_find_post_feed,
};
use crate::repository::user::repository_find_users_by_ids;
use crate::utils::r2_url::build_r2_public_url;
use mofumofu_dto::posts::{
    GetPostFeedRequest, PostFeedItem, PostFeedResponse, PostSortOrder,
};
use mofumofu_entity::hashtags::Entity as HashtagEntity;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn service_get_post_feed(
    conn: &DatabaseConnection,
    payload: GetPostFeedRequest,
) -> ServiceResult<PostFeedResponse> {
    let page = payload.page.unwrap_or(1);
    let page_size = payload.page_size.unwrap_or(20);
    let sort = payload.sort.unwrap_or(PostSortOrder::Latest);

    let filter = PostFeedFilter {
        published_at_after: payload.published_at_after,
    };

    let posts = repository_find_post_feed(conn, &filter, &sort, page, page_size).await?;

    if posts.is_empty() {
        return Ok(PostFeedResponse {
            data: Vec::new(),
            page,
            page_size,
            has_more: false,
            total_count: 0,
        });
    }

    let total_count = repository_count_post_feed(conn, &filter).await?;
    let has_more = posts.len() == page_size as usize;

    // Batch load authors
    let user_ids: Vec<_> = posts.iter().map(|p| p.user_id).collect();
    let users = repository_find_users_by_ids(conn, &user_ids).await?;
    let user_map: HashMap<_, _> = users.into_iter().map(|u| (u.id, u)).collect();

    let mut data = Vec::with_capacity(posts.len());
    for post in posts {
        let post_hashtags = repository_find_post_hashtags_by_post_id(conn, post.id).await?;
        let mut hashtag_names = Vec::with_capacity(post_hashtags.len());
        for ph in &post_hashtags {
            if let Some(hashtag) = HashtagEntity::find_by_id(ph.hashtag_id).one(conn).await? {
                hashtag_names.push(hashtag.name);
            }
        }

        let (author_handle, author_display_name, author_profile_image) = user_map
            .get(&post.user_id)
            .map(|u| {
                (
                    u.handle.clone(),
                    u.display_name.clone(),
                    u.profile_image.as_deref().map(build_r2_public_url),
                )
            })
            .unwrap_or_else(|| (String::new(), String::new(), None));

        data.push(PostFeedItem {
            id: post.id,
            user_id: post.user_id,
            author_handle,
            author_display_name,
            author_profile_image,
            title: post.title,
            slug: post.slug,
            summary: post.summary,
            thumbnail_image: post.thumbnail_image,
            hashtags: hashtag_names,
            like_count: post.like_count,
            comment_count: post.comment_count,
            view_count: post.view_count,
            published_at: post.published_at,
            created_at: post.created_at,
        });
    }

    Ok(PostFeedResponse {
        data,
        page,
        page_size,
        has_more,
        total_count,
    })
}
