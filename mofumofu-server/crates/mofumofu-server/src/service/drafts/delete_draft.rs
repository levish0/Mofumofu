use crate::repository::drafts::{repository_delete_draft, repository_get_draft_by_id};
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_delete_draft(
    conn: &DatabaseConnection,
    user_id: Uuid,
    draft_id: Uuid,
) -> ServiceResult<()> {
    let txn = conn.begin().await?;

    let draft = repository_get_draft_by_id(&txn, draft_id).await?;

    if draft.user_id != user_id {
        return Err(Errors::ForbiddenError(
            "Not the owner of this resource".to_string(),
        ));
    }

    repository_delete_draft(&txn, draft_id).await?;

    txn.commit().await?;

    Ok(())
}
