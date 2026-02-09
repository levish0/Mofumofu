use crate::entity::comments::{Column as CommentColumn, Entity as CommentEntity};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_reply_count<C>(
    conn: &C,
    parent_comment_id: Uuid,
) -> Result<u64, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let count = CommentEntity::find()
        .filter(CommentColumn::ParentId.eq(parent_comment_id))
        .filter(CommentColumn::IsDeleted.eq(false)) // 삭제된 답글은 제외
        .count(conn)
        .await?;

    Ok(count)
}
