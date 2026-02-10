use crate::repository::moderation_logs::repository_create_moderation_log;
use crate::repository::user_roles::{
    repository_create_user_role, repository_find_user_roles, repository_get_highest_user_role,
};
use mofumofu_constants::ModerationAction;
use mofumofu_dto::user_roles::{GrantRoleRequest, UserRoleResponse};
use mofumofu_entity::common::ModerationResourceType;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_grant_role(
    conn: &DatabaseConnection,
    actor_id: Uuid,
    payload: GrantRoleRequest,
) -> ServiceResult<UserRoleResponse> {
    let txn = conn.begin().await?;

    // ACL: actor must have strictly higher priority
    let actor_role = repository_get_highest_user_role(&txn, actor_id).await?;
    let actor_priority = actor_role.map(|r| r.priority()).unwrap_or(1);

    let target_current_role = repository_get_highest_user_role(&txn, payload.user_id).await?;
    let target_current_priority = target_current_role.map(|r| r.priority()).unwrap_or(1);

    // Check against both the role being granted AND the target's current highest role
    let target_priority = std::cmp::max(payload.role.priority(), target_current_priority);
    if actor_priority <= target_priority {
        return Err(Errors::CannotManageHigherOrEqualRole);
    }

    // Check target doesn't already have the role
    let target_roles = repository_find_user_roles(&txn, payload.user_id).await?;
    if target_roles.iter().any(|r| r.role == payload.role) {
        return Err(Errors::UserAlreadyHasRole);
    }

    let role =
        repository_create_user_role(&txn, payload.user_id, payload.role.clone(), Some(actor_id))
            .await?;

    let _ = repository_create_moderation_log(
        &txn,
        ModerationAction::UserGrantRole,
        actor_id,
        ModerationResourceType::User,
        Some(payload.user_id),
        Some(format!("Granted role: {:?}", payload.role)),
        None,
    )
    .await;

    txn.commit().await?;

    Ok(UserRoleResponse::from(role))
}
