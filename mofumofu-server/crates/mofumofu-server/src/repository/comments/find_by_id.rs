use mofumofu_entity::comments::{Entity as CommentEntity, Model as CommentModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait};
use uuid::Uuid;

pub async fn repository_find_comment_by_id<C>(
    conn: &C,
    id: Uuid,
) -> Result<Option<CommentModel>, Errors>
where
    C: ConnectionTrait,
{
    let comment = CommentEntity::find_by_id(id).one(conn).await?;
    Ok(comment)
}
