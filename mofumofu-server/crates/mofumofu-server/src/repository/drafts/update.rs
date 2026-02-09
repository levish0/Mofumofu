use mofumofu_entity::drafts::{
    ActiveModel as DraftActiveModel, Entity as DraftEntity, Model as DraftModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::prelude::Json;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel, Set};
use uuid::Uuid;

#[derive(Default)]
pub struct DraftUpdateParams {
    pub title: Option<Option<String>>,
    pub slug: Option<Option<String>>,
    pub content: Option<Option<String>>,
    pub metadata: Option<Option<Json>>,
}

pub async fn repository_update_draft<C>(
    conn: &C,
    draft_id: Uuid,
    params: DraftUpdateParams,
) -> Result<DraftModel, Errors>
where
    C: ConnectionTrait,
{
    let draft = DraftEntity::find_by_id(draft_id)
        .one(conn)
        .await?
        .ok_or(Errors::DraftNotFound)?;

    let mut draft_active: DraftActiveModel = draft.into_active_model();

    if let Some(title) = params.title {
        draft_active.title = Set(title);
    }
    if let Some(slug) = params.slug {
        draft_active.slug = Set(slug);
    }
    if let Some(content) = params.content {
        draft_active.content = Set(content);
    }
    if let Some(metadata) = params.metadata {
        draft_active.metadata = Set(metadata);
    }

    let updated_draft = draft_active.update(conn).await?;
    Ok(updated_draft)
}
