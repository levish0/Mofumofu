use mofumofu_entity::follows::{Column as FollowColumn, Entity as FollowEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, DeleteResult, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_delete_follow<C>(
    conn: &C,
    follower_id: Uuid,
    followee_id: Uuid,
) -> Result<DeleteResult, Errors>
where
    C: ConnectionTrait,
{
    let result = FollowEntity::delete_many()
        .filter(FollowColumn::FollowerId.eq(follower_id))
        .filter(FollowColumn::FolloweeId.eq(followee_id))
        .exec(conn)
        .await?;

    Ok(result)
}
