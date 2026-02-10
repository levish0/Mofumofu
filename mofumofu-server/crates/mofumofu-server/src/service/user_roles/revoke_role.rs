use crate::repository::moderation_logs::repository_create_moderation_log;
use crate::repository::user_roles::{
    repository_delete_user_role, repository_get_highest_user_role,
};
use mofumofu_constants::ModerationAction;
use mofumofu_entity::common::{ModerationResourceType, UserRole};
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_revoke_role(
    conn: &DatabaseConnection,
    actor_id: Uuid,
    target_user_id: Uuid,
    role: UserRole,
) -> ServiceResult<()> {
    let txn = conn.begin().await?;

    // ACL: actor must have strictly higher priority than the role being revoked
    let actor_role = repository_get_highest_user_role(&txn, actor_id).await?;
    let actor_priority = actor_role.map(|r| r.priority()).unwrap_or(1);

    if actor_priority <= role.priority() {
        return Err(Errors::CannotManageHigherOrEqualRole);
    }

    let result = repository_delete_user_role(&txn, target_user_id, role.clone()).await?;
    if result.rows_affected == 0 {
        return Err(Errors::UserDoesNotHaveRole);
    }

    let _ = repository_create_moderation_log(
        &txn,
        ModerationAction::UserRevokeRole,
        actor_id,
        ModerationResourceType::User,
        Some(target_user_id),
        Some(format!("Revoked role: {:?}", role)),
        None,
    )
    .await;

    txn.commit().await?;

    Ok(())
}
