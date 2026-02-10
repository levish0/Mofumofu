use mofumofu_dto::pagination::CursorDirection;
use mofumofu_entity::follows::{
    Column as FollowColumn, Entity as FollowEntity, Model as FollowModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

pub async fn repository_find_followers<C>(
    conn: &C,
    user_id: Uuid,
    cursor_id: Option<Uuid>,
    cursor_direction: Option<CursorDirection>,
    limit: u64,
) -> Result<Vec<FollowModel>, Errors>
where
    C: ConnectionTrait,
{
    let mut query = FollowEntity::find().filter(FollowColumn::FolloweeId.eq(user_id));

    if let Some(id) = cursor_id {
        let direction = cursor_direction.unwrap_or(CursorDirection::Older);
        query = match direction {
            CursorDirection::Older => query
                .filter(FollowColumn::Id.lt(id))
                .order_by_desc(FollowColumn::Id),
            CursorDirection::Newer => query
                .filter(FollowColumn::Id.gt(id))
                .order_by_asc(FollowColumn::Id),
        };
    } else {
        query = query.order_by_desc(FollowColumn::Id);
    }

    let follows = query.limit(limit).all(conn).await?;
    Ok(follows)
}
