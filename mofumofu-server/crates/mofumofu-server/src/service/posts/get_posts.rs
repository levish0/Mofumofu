use std::collections::HashMap;

use crate::repository::post_hashtags::repository_find_post_hashtags_by_post_id;
use crate::repository::posts::{
    PostFilter, repository_exists_newer_post, repository_exists_older_post, repository_find_posts,
};
use crate::repository::user::repository_find_users_by_ids;
use crate::utils::r2_url::build_r2_public_url;
use mofumofu_dto::pagination::CursorDirection;
use mofumofu_dto::posts::{GetPostsRequest, PostAuthor, PostListResponse, PostResponse};
use mofumofu_entity::hashtags::Entity as HashtagEntity;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn service_get_posts(
    conn: &DatabaseConnection,
    payload: GetPostsRequest,
) -> ServiceResult<PostListResponse> {
    let is_newer = payload.cursor_direction == Some(CursorDirection::Newer);

    let filter = PostFilter {
        user_id: payload.user_id,
        published_only: payload.published_only,
    };

    let mut posts = repository_find_posts(
        conn,
        &filter,
        payload.cursor_id,
        payload.cursor_direction,
        payload.limit,
    )
    .await?;

    let (has_newer, has_older) = if posts.is_empty() {
        (false, false)
    } else {
        let first_id = posts.first().unwrap().id;
        let last_id = posts.last().unwrap().id;
        if is_newer {
            let has_newer = repository_exists_newer_post(conn, &filter, last_id).await?;
            let has_older = repository_exists_older_post(conn, &filter, first_id).await?;
            (has_newer, has_older)
        } else {
            let has_newer = repository_exists_newer_post(conn, &filter, first_id).await?;
            let has_older = repository_exists_older_post(conn, &filter, last_id).await?;
            (has_newer, has_older)
        }
    };

    if is_newer {
        posts.reverse();
    }

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

        let author = user_map
            .get(&post.user_id)
            .map(|u| PostAuthor {
                id: u.id,
                handle: u.handle.clone(),
                display_name: u.display_name.clone(),
                profile_image: u.profile_image.as_deref().map(build_r2_public_url),
            })
            .unwrap_or_else(|| PostAuthor {
                id: post.user_id,
                handle: String::new(),
                display_name: String::new(),
                profile_image: None,
            });

        data.push(PostResponse {
            id: post.id,
            user_id: post.user_id,
            author,
            title: post.title,
            slug: post.slug,
            thumbnail_image: post.thumbnail_image,
            summary: post.summary,
            content: post.content,
            render: post.render,
            toc: post.toc,
            like_count: post.like_count,
            comment_count: post.comment_count,
            view_count: post.view_count,
            hashtags: hashtag_names,
            published_at: post.published_at,
            created_at: post.created_at,
            updated_at: post.updated_at,
        });
    }

    Ok(PostListResponse {
        data,
        has_newer,
        has_older,
    })
}
