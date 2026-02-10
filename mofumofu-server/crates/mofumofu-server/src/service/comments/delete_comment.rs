use crate::repository::comments::{repository_get_comment_by_id, repository_soft_delete_comment};
use crate::repository::posts::repository_decrement_post_comment_count;
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_delete_comment(
    conn: &DatabaseConnection,
    user_id: Uuid,
    comment_id: Uuid,
) -> ServiceResult<()> {
    let txn = conn.begin().await?;

    let comment = repository_get_comment_by_id(&txn, comment_id).await?;

    if comment.user_id != user_id {
        return Err(Errors::ForbiddenError(
            "Not the owner of this resource".to_string(),
        ));
    }

    repository_soft_delete_comment(&txn, comment_id).await?;
    repository_decrement_post_comment_count(&txn, comment.post_id).await?;

    txn.commit().await?;

    Ok(())
}
