use mofumofu_entity::drafts::{Entity as DraftEntity, Model as DraftModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait};
use uuid::Uuid;

pub async fn repository_get_draft_by_id<C>(conn: &C, id: Uuid) -> Result<DraftModel, Errors>
where
    C: ConnectionTrait,
{
    let draft = DraftEntity::find_by_id(id).one(conn).await?;
    draft.ok_or(Errors::DraftNotFound)
}
