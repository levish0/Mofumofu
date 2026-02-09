use crate::entity::follows::{ActiveModel as FollowsActiveModel, Model as FollowsModel};
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_follow<C>(
    conn: &C,
    follower_id: Uuid,
    followee_id: Uuid,
) -> Result<FollowsModel, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let new_follow = FollowsActiveModel {
        id: Default::default(),
        follower_id: Set(follower_id),
        followee_id: Set(followee_id),
    };

    let created_follow = new_follow.insert(conn).await?;
    Ok(created_follow)
}
