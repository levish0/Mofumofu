use mofumofu_entity::comments::{Entity as CommentEntity, Model as CommentModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait};
use uuid::Uuid;

pub async fn repository_get_comment_by_id<C>(conn: &C, id: Uuid) -> Result<CommentModel, Errors>
where
    C: ConnectionTrait,
{
    let comment = CommentEntity::find_by_id(id).one(conn).await?;
    comment.ok_or(Errors::CommentNotFound)
}
