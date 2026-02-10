use crate::repository::moderation_logs::repository_create_moderation_log;
use crate::repository::user_bans::{repository_create_user_ban, repository_find_user_bans};
use crate::repository::user_roles::repository_get_highest_user_role;
use mofumofu_constants::ModerationAction;
use mofumofu_dto::user_bans::{BanUserRequest, UserBanResponse};
use mofumofu_entity::common::ModerationResourceType;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_ban_user(
    conn: &DatabaseConnection,
    actor_id: Uuid,
    payload: BanUserRequest,
) -> ServiceResult<UserBanResponse> {
    let txn = conn.begin().await?;

    // ACL: actor must have strictly higher priority than target
    let actor_role = repository_get_highest_user_role(&txn, actor_id).await?;
    let actor_priority = actor_role.map(|r| r.priority()).unwrap_or(1);

    let target_role = repository_get_highest_user_role(&txn, payload.user_id).await?;
    let target_priority = target_role.map(|r| r.priority()).unwrap_or(1);

    if actor_priority <= target_priority {
        return Err(Errors::CannotManageHigherOrEqualRole);
    }

    // Check not already banned (active ban)
    let existing_bans = repository_find_user_bans(&txn, payload.user_id).await?;
    let has_active_ban = existing_bans.iter().any(|b| {
        b.expires_at
            .map(|exp| exp > chrono::Utc::now())
            .unwrap_or(true)
    });
    if has_active_ban {
        return Err(Errors::UserAlreadyBanned);
    }

    let ban = repository_create_user_ban(
        &txn,
        payload.user_id,
        actor_id,
        payload.reason.clone(),
        payload.expires_at,
    )
    .await?;

    let _ = repository_create_moderation_log(
        &txn,
        ModerationAction::UserBan,
        actor_id,
        ModerationResourceType::User,
        Some(payload.user_id),
        Some(payload.reason),
        None,
    )
    .await;

    txn.commit().await?;

    Ok(UserBanResponse::from(ban))
}
