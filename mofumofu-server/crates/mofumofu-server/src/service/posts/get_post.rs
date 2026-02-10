use crate::repository::post_hashtags::repository_find_post_hashtags_by_post_id;
use crate::repository::posts::repository_get_post_by_id;
use mofumofu_dto::posts::PostResponse;
use mofumofu_entity::hashtags::Entity as HashtagEntity;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

pub async fn service_get_post(
    conn: &DatabaseConnection,
    post_id: Uuid,
) -> ServiceResult<PostResponse> {
    let post = repository_get_post_by_id(conn, post_id).await?;

    let post_hashtags = repository_find_post_hashtags_by_post_id(conn, post_id).await?;
    let mut hashtag_names = Vec::with_capacity(post_hashtags.len());
    for ph in &post_hashtags {
        if let Some(hashtag) = HashtagEntity::find_by_id(ph.hashtag_id).one(conn).await? {
            hashtag_names.push(hashtag.name);
        }
    }

    Ok(PostResponse::from_model(post, hashtag_names))
}
