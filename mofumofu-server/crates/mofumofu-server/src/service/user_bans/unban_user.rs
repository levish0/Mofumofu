use crate::repository::moderation_logs::repository_create_moderation_log;
use crate::repository::user_bans::{repository_delete_user_ban, repository_find_user_bans};
use crate::repository::user_roles::repository_get_highest_user_role;
use mofumofu_constants::ModerationAction;
use mofumofu_entity::common::ModerationResourceType;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_unban_user(
    conn: &DatabaseConnection,
    actor_id: Uuid,
    target_user_id: Uuid,
) -> ServiceResult<()> {
    let txn = conn.begin().await?;

    // ACL: actor must have strictly higher priority than target
    let actor_role = repository_get_highest_user_role(&txn, actor_id).await?;
    let actor_priority = actor_role.map(|r| r.priority()).unwrap_or(1);

    let target_role = repository_get_highest_user_role(&txn, target_user_id).await?;
    let target_priority = target_role.map(|r| r.priority()).unwrap_or(1);

    if actor_priority <= target_priority {
        return Err(Errors::CannotManageHigherOrEqualRole);
    }

    // Find active ban
    let bans = repository_find_user_bans(&txn, target_user_id).await?;
    let active_ban = bans.into_iter().find(|b| {
        b.expires_at
            .map(|exp| exp > chrono::Utc::now())
            .unwrap_or(true)
    });

    let ban = active_ban.ok_or(Errors::UserNotBanned)?;

    repository_delete_user_ban(&txn, ban.id).await?;

    let _ = repository_create_moderation_log(
        &txn,
        ModerationAction::UserUnban,
        actor_id,
        ModerationResourceType::User,
        Some(target_user_id),
        None,
        None,
    )
    .await;

    txn.commit().await?;

    Ok(())
}
