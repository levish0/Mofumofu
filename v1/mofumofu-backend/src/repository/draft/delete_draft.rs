use crate::entity::drafts::Entity as DraftEntity;
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait};
use uuid::Uuid;

pub async fn repository_delete_draft<C>(
    txn: &C,
    draft_id: &Uuid,
    user_id: &Uuid,
) -> Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    DraftEntity::delete_many()
        .filter(crate::entity::drafts::Column::Id.eq(*draft_id))
        .filter(crate::entity::drafts::Column::UserId.eq(*user_id))
        .exec(txn)
        .await?;

    Ok(())
}
