use crate::dto::draft::request::update_draft::UpdateDraftRequest;
use crate::entity::drafts::{ActiveModel as DraftActiveModel, Model as DraftModel};
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn repository_update_draft<C>(
    txn: &C,
    draft_id: &Uuid,
    payload: UpdateDraftRequest,
) -> Result<DraftModel, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let mut draft = DraftActiveModel {
        id: Set(*draft_id),
        updated_at: Set(Some(Utc::now())),
        ..Default::default()
    };

    if let Some(title) = payload.title {
        draft.title = Set(Some(title));
    }
    if let Some(thumbnail_image) = payload.thumbnail_image {
        draft.thumbnail_image = Set(Some(thumbnail_image));
    }
    if let Some(summary) = payload.summary {
        draft.summary = Set(Some(summary));
    }
    if let Some(content) = payload.content {
        draft.content = Set(Some(content));
    }

    let updated_draft = draft.update(txn).await?;

    Ok(updated_draft)
}
