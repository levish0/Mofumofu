use crate::repository::draft::delete_draft::repository_delete_draft;
use crate::repository::draft::get_draft_by_id::repository_get_draft_by_id;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use tracing::info;
use uuid::Uuid;

pub async fn service_delete_draft<C>(
    conn: &C,
    draft_id: &Uuid,
    user_uuid: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    // 드래프트 존재 확인 및 권한 체크
    let existing_draft = repository_get_draft_by_id(conn, draft_id, user_uuid).await?;
    if existing_draft.is_none() {
        return Err(Errors::DraftNotFound);
    }

    let txn = conn.begin().await?;

    repository_delete_draft(&txn, draft_id, user_uuid).await?;

    txn.commit().await?;

    info!("드래프트 삭제 완료 (draft_id: {})", draft_id);

    Ok(())
}
