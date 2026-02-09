use mofumofu_entity::common::LikeTargetType;
use mofumofu_entity::likes::{ActiveModel as LikeActiveModel, Model as LikeModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_like<C>(
    conn: &C,
    user_id: Uuid,
    target_type: LikeTargetType,
    target_id: Uuid,
) -> Result<LikeModel, Errors>
where
    C: ConnectionTrait,
{
    let new_like = LikeActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        target_type: Set(target_type),
        target_id: Set(target_id),
        created_at: Default::default(),
    };

    let like = new_like.insert(conn).await?;
    Ok(like)
}
