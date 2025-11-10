use crate::dto::draft::request::update_draft::UpdateDraftRequest;
use crate::repository::draft::get_draft_by_id::repository_get_draft_by_id;
use crate::repository::draft::update_draft::repository_update_draft;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use tracing::info;
use uuid::Uuid;

pub async fn service_update_draft<C>(
    conn: &C,
    payload: UpdateDraftRequest,
    user_uuid: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    // 드래프트 존재 확인 및 권한 체크
    let existing_draft = repository_get_draft_by_id(conn, &payload.draft_id, user_uuid).await?;
    if existing_draft.is_none() {
        return Err(Errors::DraftNotFound);
    }

    let txn = conn.begin().await?;

    let draft_id = payload.draft_id;
    repository_update_draft(&txn, &draft_id, payload).await?;

    txn.commit().await?;

    info!("드래프트 수정 완료 (draft_id: {})", draft_id);

    Ok(())
}
