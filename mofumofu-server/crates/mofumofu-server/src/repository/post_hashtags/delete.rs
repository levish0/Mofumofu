use mofumofu_entity::post_hashtags::{Column as PostHashtagColumn, Entity as PostHashtagEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, DeleteResult, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_delete_post_hashtag<C>(
    conn: &C,
    post_id: Uuid,
    hashtag_id: Uuid,
) -> Result<DeleteResult, Errors>
where
    C: ConnectionTrait,
{
    let result = PostHashtagEntity::delete_many()
        .filter(PostHashtagColumn::PostId.eq(post_id))
        .filter(PostHashtagColumn::HashtagId.eq(hashtag_id))
        .exec(conn)
        .await?;

    Ok(result)
}
