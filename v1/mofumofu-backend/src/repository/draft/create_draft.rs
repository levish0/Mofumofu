use crate::dto::draft::request::create_draft::CreateDraftRequest;
use crate::entity::drafts::{ActiveModel as DraftActiveModel, Model as DraftModel};
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn repository_create_draft<C>(
    txn: &C,
    payload: CreateDraftRequest,
    user_uuid: &Uuid,
) -> Result<DraftModel, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_draft = DraftActiveModel {
        id: Default::default(),
        user_id: Set(*user_uuid),
        title: Set(payload.title),
        thumbnail_image: Set(payload.thumbnail_image),
        summary: Set(payload.summary),
        content: Set(payload.content),
        slug: Set(payload.slug),
        created_at: Set(Utc::now()),
        updated_at: Set(Some(Utc::now())),
    };

    let created_draft = new_draft.insert(txn).await?;

    Ok(created_draft)
}
