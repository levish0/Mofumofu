use crate::dto::post::response::post_edit_info::PostEditInfoResponse;
use crate::repository::hashtag::get_hashtags_by_post::repository_get_hashtags_by_post;
use crate::repository::post::get_post_by_user_and_slug::repository_get_post_by_user_and_slug;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_get_post_for_edit<C>(
    conn: &C,
    slug: &str,
    user_uuid: &Uuid,
) -> ServiceResult<PostEditInfoResponse>
where
    C: ConnectionTrait,
{
    let post = repository_get_post_by_user_and_slug(conn, user_uuid, slug).await?;

    let hashtags = repository_get_hashtags_by_post(conn, post.id).await?;
    let tag_names: Vec<String> = hashtags.into_iter().map(|tag| tag.name).collect();

    let response = PostEditInfoResponse {
        id: post.id,
        title: post.title,
        summary: post.summary,
        content: post.content,
        thumbnail_image: post.thumbnail_image,
        created_at: post.created_at,
        updated_at: post.updated_at,
        slug: post.slug,
        tags: tag_names,
    };

    Ok(response)
}
