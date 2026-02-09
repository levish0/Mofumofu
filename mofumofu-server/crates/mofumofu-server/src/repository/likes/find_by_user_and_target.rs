use mofumofu_entity::common::LikeTargetType;
use mofumofu_entity::likes::{Column as LikeColumn, Entity as LikeEntity, Model as LikeModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_find_like<C>(
    conn: &C,
    user_id: Uuid,
    target_type: LikeTargetType,
    target_id: Uuid,
) -> Result<Option<LikeModel>, Errors>
where
    C: ConnectionTrait,
{
    let like = LikeEntity::find()
        .filter(LikeColumn::UserId.eq(user_id))
        .filter(LikeColumn::TargetType.eq(target_type))
        .filter(LikeColumn::TargetId.eq(target_id))
        .one(conn)
        .await?;

    Ok(like)
}
