use mofumofu_entity::common::LikeTargetType;
use mofumofu_entity::likes::{Column as LikeColumn, Entity as LikeEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, DeleteResult, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_delete_like<C>(
    conn: &C,
    user_id: Uuid,
    target_type: LikeTargetType,
    target_id: Uuid,
) -> Result<DeleteResult, Errors>
where
    C: ConnectionTrait,
{
    let result = LikeEntity::delete_many()
        .filter(LikeColumn::UserId.eq(user_id))
        .filter(LikeColumn::TargetType.eq(target_type))
        .filter(LikeColumn::TargetId.eq(target_id))
        .exec(conn)
        .await?;

    Ok(result)
}
