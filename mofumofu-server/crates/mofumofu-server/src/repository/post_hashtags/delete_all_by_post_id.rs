use mofumofu_entity::post_hashtags::{Column as PostHashtagColumn, Entity as PostHashtagEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, DeleteResult, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_delete_all_post_hashtags_by_post_id<C>(
    conn: &C,
    post_id: Uuid,
) -> Result<DeleteResult, Errors>
where
    C: ConnectionTrait,
{
    let result = PostHashtagEntity::delete_many()
        .filter(PostHashtagColumn::PostId.eq(post_id))
        .exec(conn)
        .await?;

    Ok(result)
}
