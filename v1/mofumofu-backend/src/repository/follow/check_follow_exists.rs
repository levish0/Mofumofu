use crate::entity::follows::{Column as FollowsColumn, Entity as FollowsEntity};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_check_follow_exists<C>(
    conn: &C,
    follower_id: Uuid,
    followee_id: Uuid,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let existing_follow = FollowsEntity::find()
        .filter(FollowsColumn::FollowerId.eq(follower_id))
        .filter(FollowsColumn::FolloweeId.eq(followee_id))
        .one(conn)
        .await?;

    Ok(existing_follow.is_some())
}
