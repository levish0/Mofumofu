use crate::entity::common::{ActionType, TargetType};
use crate::repository::comment::get_comment_by_id::repository_get_comment_by_id;
use crate::repository::comment::update_like_count::repository_decrement_comment_like_count;
use crate::repository::like::delete_like::repository_delete_like_by_comment_id;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use uuid::Uuid;

pub async fn service_delete_comment_like<C>(
    conn: &C,
    user_id: &Uuid,
    comment_id: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 댓글 존재 확인
    let comment = repository_get_comment_by_id(&txn, *comment_id)
        .await?
        .ok_or(Errors::CommentNotFound)?;

    // 좋아요 삭제
    let deleted = repository_delete_like_by_comment_id(&txn, *user_id, *comment_id).await?;

    if !deleted {
        return Err(Errors::LikeNotFound);
    }

    // 댓글 좋아요 수 감소
    repository_decrement_comment_like_count(&txn, comment_id).await?;

    txn.commit().await?;

    // 좋아요 삭제 이벤트 로깅
    repository_log_event(
        conn,
        Some(*user_id),
        ActionType::LikeDeleted,
        Some(comment.post_id), // 포스트 ID로 로깅
        Some(TargetType::Comment),
        None,
    )
    .await;

    Ok(())
}
