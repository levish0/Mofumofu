use mofumofu_entity::follows::{
    Column as FollowColumn, Entity as FollowEntity, Model as FollowModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_find_follow<C>(
    conn: &C,
    follower_id: Uuid,
    followee_id: Uuid,
) -> Result<Option<FollowModel>, Errors>
where
    C: ConnectionTrait,
{
    let follow = FollowEntity::find()
        .filter(FollowColumn::FollowerId.eq(follower_id))
        .filter(FollowColumn::FolloweeId.eq(followee_id))
        .one(conn)
        .await?;

    Ok(follow)
}
