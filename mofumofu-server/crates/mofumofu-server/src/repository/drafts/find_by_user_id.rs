use mofumofu_entity::drafts::{Column as DraftColumn, Entity as DraftEntity, Model as DraftModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

pub async fn repository_find_drafts_by_user_id<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<Vec<DraftModel>, Errors>
where
    C: ConnectionTrait,
{
    let drafts = DraftEntity::find()
        .filter(DraftColumn::UserId.eq(user_id))
        .order_by_desc(DraftColumn::UpdatedAt)
        .all(conn)
        .await?;

    Ok(drafts)
}
