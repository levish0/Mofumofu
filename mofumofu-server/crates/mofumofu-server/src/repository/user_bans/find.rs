use mofumofu_entity::user_bans::{
    Column as UserBanColumn, Entity as UserBanEntity, Model as UserBanModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_find_user_bans<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<Vec<UserBanModel>, Errors>
where
    C: ConnectionTrait,
{
    let bans = UserBanEntity::find()
        .filter(UserBanColumn::UserId.eq(user_id))
        .all(conn)
        .await?;

    Ok(bans)
}
