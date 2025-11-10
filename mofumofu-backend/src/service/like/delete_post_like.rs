use crate::entity::common::{ActionType, TargetType};
use crate::repository::like::delete_like::repository_delete_like_by_post_id;
use crate::repository::post::get_post_by_uuid::repository_get_post_by_uuid;
use crate::repository::post::update_like_count::repository_decrement_post_like_count;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use uuid::Uuid;

pub async fn service_delete_post_like<C>(
    conn: &C,
    user_id: &Uuid,
    post_id: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 포스트 존재 확인
    let _post = repository_get_post_by_uuid(&txn, post_id).await?;

    // 좋아요 삭제
    let deleted = repository_delete_like_by_post_id(&txn, *user_id, *post_id).await?;

    if !deleted {
        return Err(Errors::LikeNotFound);
    }

    // 포스트 좋아요 개수 감소
    repository_decrement_post_like_count(&txn, *post_id).await?;

    txn.commit().await?;

    // 좋아요 삭제 이벤트 로깅
    repository_log_event(
        conn,
        Some(*user_id),
        ActionType::LikeDeleted,
        Some(*post_id),
        Some(TargetType::Post),
        None,
    )
    .await;

    Ok(())
}
