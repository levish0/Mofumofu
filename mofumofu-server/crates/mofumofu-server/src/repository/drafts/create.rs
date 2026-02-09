use mofumofu_entity::drafts::{ActiveModel as DraftActiveModel, Model as DraftModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_draft<C>(conn: &C, user_id: Uuid) -> Result<DraftModel, Errors>
where
    C: ConnectionTrait,
{
    let new_draft = DraftActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        title: Set(None),
        slug: Set(None),
        content: Set(None),
        metadata: Set(None),
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    let draft = new_draft.insert(conn).await?;
    Ok(draft)
}
