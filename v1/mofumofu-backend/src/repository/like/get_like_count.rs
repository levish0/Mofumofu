use crate::entity::likes::{Column as LikesColumn, Entity as LikesEntity};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_like_count_by_post_id<C>(
    conn: &C,
    post_id: Uuid,
) -> Result<u64, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let count = LikesEntity::find()
        .filter(LikesColumn::PostId.eq(post_id))
        .count(conn)
        .await?;

    Ok(count)
}

pub async fn repository_get_like_count_by_comment_id<C>(
    conn: &C,
    comment_id: Uuid,
) -> Result<u64, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let count = LikesEntity::find()
        .filter(LikesColumn::CommentId.eq(comment_id))
        .count(conn)
        .await?;

    Ok(count)
}
