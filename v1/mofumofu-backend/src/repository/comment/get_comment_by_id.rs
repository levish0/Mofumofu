use crate::entity::comments::{
    Column as CommentColumn, Entity as CommentEntity, Model as CommentModel,
};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_comment_by_id<C>(
    conn: &C,
    comment_id: Uuid,
) -> Result<Option<CommentModel>, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let comment = CommentEntity::find()
        .filter(CommentColumn::Id.eq(comment_id))
        .one(conn)
        .await?;

    Ok(comment)
}
