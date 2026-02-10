use crate::repository::drafts::{
    DraftUpdateParams, repository_get_draft_by_id, repository_update_draft,
};
use mofumofu_dto::drafts::{DraftResponse, UpdateDraftRequest};
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_update_draft(
    conn: &DatabaseConnection,
    user_id: Uuid,
    draft_id: Uuid,
    payload: UpdateDraftRequest,
) -> ServiceResult<DraftResponse> {
    let txn = conn.begin().await?;

    let draft = repository_get_draft_by_id(&txn, draft_id).await?;

    if draft.user_id != user_id {
        return Err(Errors::ForbiddenError(
            "Not the owner of this resource".to_string(),
        ));
    }

    let params = DraftUpdateParams {
        title: payload.title,
        slug: payload.slug,
        content: payload.content,
        metadata: payload.metadata,
    };

    let updated_draft = repository_update_draft(&txn, draft_id, params).await?;

    txn.commit().await?;

    Ok(DraftResponse::from(updated_draft))
}
