use mofumofu_entity::drafts::Entity as DraftEntity;
use mofumofu_errors::errors::Errors;
use sea_orm::{ConnectionTrait, DeleteResult, EntityTrait, ModelTrait};
use uuid::Uuid;

pub async fn repository_delete_draft<C>(conn: &C, draft_id: Uuid) -> Result<DeleteResult, Errors>
where
    C: ConnectionTrait,
{
    let draft = DraftEntity::find_by_id(draft_id)
        .one(conn)
        .await?
        .ok_or(Errors::DraftNotFound)?;

    let result = draft.delete(conn).await?;
    Ok(result)
}
