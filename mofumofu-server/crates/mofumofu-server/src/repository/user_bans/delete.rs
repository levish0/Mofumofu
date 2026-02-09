use mofumofu_entity::user_bans::Entity as UserBanEntity;
use mofumofu_errors::errors::Errors;
use sea_orm::{ConnectionTrait, DeleteResult, EntityTrait, ModelTrait};
use uuid::Uuid;

pub async fn repository_delete_user_ban<C>(conn: &C, id: Uuid) -> Result<DeleteResult, Errors>
where
    C: ConnectionTrait,
{
    let ban = UserBanEntity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(Errors::NotFound("UserBan not found".to_string()))?;

    let result = ban.delete(conn).await?;
    Ok(result)
}
