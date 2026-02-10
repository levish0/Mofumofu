use mofumofu_entity::comments::{Column as CommentColumn, Entity as CommentEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

pub async fn repository_exists_newer_comment<C>(
    conn: &C,
    post_id: Uuid,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let count = CommentEntity::find()
        .filter(CommentColumn::PostId.eq(post_id))
        .filter(CommentColumn::Id.gt(cursor_id))
        .limit(1)
        .count(conn)
        .await?;

    Ok(count > 0)
}
