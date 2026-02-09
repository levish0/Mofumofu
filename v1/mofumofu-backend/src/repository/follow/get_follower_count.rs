use crate::entity::follows::{Column as FollowsColumn, Entity as FollowsEntity};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_follower_count<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<u64, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let count = FollowsEntity::find()
        .filter(FollowsColumn::FolloweeId.eq(user_id))
        .count(conn)
        .await?;

    Ok(count)
}
