use crate::entity::drafts::{Entity as DraftEntity, Model as DraftModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, Order, QueryFilter, QueryOrder};
use uuid::Uuid;

pub async fn repository_get_drafts_by_user<C>(
    db: &C,
    user_id: &Uuid,
) -> Result<Vec<DraftModel>, Errors>
where
    C: ConnectionTrait,
{
    let drafts = DraftEntity::find()
        .filter(crate::entity::drafts::Column::UserId.eq(*user_id))
        .order_by(crate::entity::drafts::Column::UpdatedAt, Order::Desc)
        .all(db)
        .await?;

    Ok(drafts)
}
