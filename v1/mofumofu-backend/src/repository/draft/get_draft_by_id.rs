use crate::entity::drafts::{Entity as DraftEntity, Model as DraftModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_draft_by_id<C>(
    db: &C,
    draft_id: &Uuid,
    user_id: &Uuid,
) -> Result<Option<DraftModel>, Errors>
where
    C: ConnectionTrait,
{
    let draft = DraftEntity::find()
        .filter(crate::entity::drafts::Column::Id.eq(*draft_id))
        .filter(crate::entity::drafts::Column::UserId.eq(*user_id))
        .one(db)
        .await?;

    Ok(draft)
}
