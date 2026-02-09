use mofumofu_dto::pagination::CursorDirection;
use mofumofu_entity::comments::{
    Column as CommentColumn, Entity as CommentEntity, Model as CommentModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

pub async fn repository_find_comments_by_post_id<C>(
    conn: &C,
    post_id: Uuid,
    cursor_id: Option<Uuid>,
    cursor_direction: Option<CursorDirection>,
    limit: u64,
) -> Result<Vec<CommentModel>, Errors>
where
    C: ConnectionTrait,
{
    let mut query = CommentEntity::find().filter(CommentColumn::PostId.eq(post_id));

    if let Some(id) = cursor_id {
        let direction = cursor_direction.unwrap_or(CursorDirection::Older);
        query = match direction {
            CursorDirection::Older => query
                .filter(CommentColumn::Id.lt(id))
                .order_by_desc(CommentColumn::Id),
            CursorDirection::Newer => query
                .filter(CommentColumn::Id.gt(id))
                .order_by_asc(CommentColumn::Id),
        };
    } else {
        query = query.order_by_desc(CommentColumn::Id);
    }

    let comments = query.limit(limit).all(conn).await?;

    Ok(comments)
}
