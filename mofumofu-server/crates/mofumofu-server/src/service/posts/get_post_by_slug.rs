use crate::repository::post_hashtags::repository_find_post_hashtags_by_post_id;
use crate::repository::posts::repository_find_post_by_user_id_and_slug;
use crate::repository::user::repository_find_user_by_handle;
use mofumofu_dto::posts::{PostAuthor, PostResponse};
use mofumofu_entity::hashtags::Entity as HashtagEntity;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn service_get_post_by_slug(
    conn: &DatabaseConnection,
    handle: String,
    slug: String,
) -> ServiceResult<PostResponse> {
    let user = repository_find_user_by_handle(conn, handle)
        .await?
        .ok_or(Errors::UserNotFound)?;

    let post = repository_find_post_by_user_id_and_slug(conn, user.id, slug)
        .await?
        .ok_or(Errors::PostNotFound)?;

    let author = PostAuthor {
        id: user.id,
        handle: user.handle,
        display_name: user.display_name,
        profile_image: user.profile_image,
    };

    let post_hashtags = repository_find_post_hashtags_by_post_id(conn, post.id).await?;
    let mut hashtag_names = Vec::with_capacity(post_hashtags.len());
    for ph in &post_hashtags {
        if let Some(hashtag) = HashtagEntity::find_by_id(ph.hashtag_id).one(conn).await? {
            hashtag_names.push(hashtag.name);
        }
    }

    Ok(PostResponse {
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
    })
}
