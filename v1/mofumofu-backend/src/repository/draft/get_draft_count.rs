use crate::entity::drafts::Entity as DraftEntity;
use crate::service::error::errors::Errors;
use sea_orm::PaginatorTrait;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_draft_count<C>(db: &C, user_id: &Uuid) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let count = DraftEntity::find()
        .filter(crate::entity::drafts::Column::UserId.eq(*user_id))
        .count(db)
        .await?;

    Ok(count)
}
