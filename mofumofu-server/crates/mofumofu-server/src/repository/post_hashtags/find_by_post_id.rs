use mofumofu_entity::post_hashtags::{
    Column as PostHashtagColumn, Entity as PostHashtagEntity, Model as PostHashtagModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_find_post_hashtags_by_post_id<C>(
    conn: &C,
    post_id: Uuid,
) -> Result<Vec<PostHashtagModel>, Errors>
where
    C: ConnectionTrait,
{
    let post_hashtags = PostHashtagEntity::find()
        .filter(PostHashtagColumn::PostId.eq(post_id))
        .all(conn)
        .await?;

    Ok(post_hashtags)
}
