use mofumofu_entity::follows::{ActiveModel as FollowActiveModel, Model as FollowModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_follow<C>(
    conn: &C,
    follower_id: Uuid,
    followee_id: Uuid,
) -> Result<FollowModel, Errors>
where
    C: ConnectionTrait,
{
    let new_follow = FollowActiveModel {
        id: Default::default(),
        follower_id: Set(follower_id),
        followee_id: Set(followee_id),
        created_at: Default::default(),
    };

    let follow = new_follow.insert(conn).await?;
    Ok(follow)
}
