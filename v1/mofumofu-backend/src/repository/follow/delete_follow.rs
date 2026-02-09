use crate::entity::follows::{
    ActiveModel as FollowsActiveModel, Column as FollowsColumn, Entity as FollowsEntity,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_delete_follow<C>(
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

    match existing_follow {
        Some(follow_record) => {
            let follow_active_model: FollowsActiveModel = follow_record.into();
            follow_active_model.delete(conn).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}
