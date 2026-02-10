use mofumofu_entity::follows::{Column as FollowColumn, Entity as FollowEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

pub async fn repository_exists_newer_follower<C>(
    conn: &C,
    user_id: Uuid,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let count = FollowEntity::find()
        .filter(FollowColumn::FolloweeId.eq(user_id))
        .filter(FollowColumn::Id.gt(cursor_id))
        .limit(1)
        .count(conn)
        .await?;
    Ok(count > 0)
}

pub async fn repository_exists_newer_following<C>(
    conn: &C,
    user_id: Uuid,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let count = FollowEntity::find()
        .filter(FollowColumn::FollowerId.eq(user_id))
        .filter(FollowColumn::Id.gt(cursor_id))
        .limit(1)
        .count(conn)
        .await?;
    Ok(count > 0)
}
