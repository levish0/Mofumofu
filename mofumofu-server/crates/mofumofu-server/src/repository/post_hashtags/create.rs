use mofumofu_entity::post_hashtags::{
    ActiveModel as PostHashtagActiveModel, Model as PostHashtagModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_post_hashtag<C>(
    conn: &C,
    post_id: Uuid,
    hashtag_id: Uuid,
) -> Result<PostHashtagModel, Errors>
where
    C: ConnectionTrait,
{
    let new_post_hashtag = PostHashtagActiveModel {
        post_id: Set(post_id),
        hashtag_id: Set(hashtag_id),
    };

    let post_hashtag = new_post_hashtag.insert(conn).await?;
    Ok(post_hashtag)
}
