use mofumofu_entity::user_bans::{ActiveModel as UserBanActiveModel, Model as UserBanModel};
use mofumofu_errors::errors::Errors;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_user_ban<C>(
    conn: &C,
    user_id: Uuid,
    banned_by: Uuid,
    reason: String,
    expires_at: Option<DateTimeUtc>,
) -> Result<UserBanModel, Errors>
where
    C: ConnectionTrait,
{
    let new_ban = UserBanActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        banned_by: Set(banned_by),
        reason: Set(reason),
        expires_at: Set(expires_at),
        created_at: Default::default(),
    };

    let ban = new_ban.insert(conn).await?;
    Ok(ban)
}
