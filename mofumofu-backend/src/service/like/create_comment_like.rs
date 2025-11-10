use crate::entity::common::{ActionType, TargetType};
use crate::repository::comment::get_comment_by_id::repository_get_comment_by_id;
use crate::repository::comment::update_like_count::repository_increment_comment_like_count;
use crate::repository::like::check_like_status::repository_check_like_status_by_comment_id;
use crate::repository::like::create_like::repository_create_like_by_comment_id;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use uuid::Uuid;

pub async fn service_create_comment_like<C>(
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

    // 삭제된 댓글에는 좋아요를 누를 수 없음
    if comment.is_deleted {
        return Err(Errors::CommentNotFound);
    }

    // 이미 좋아요가 있는지 확인
    let already_liked =
        repository_check_like_status_by_comment_id(&txn, user_id, comment_id).await?;
    if already_liked {
        return Err(Errors::LikeAlreadyExists);
    }

    // 좋아요 생성
    let _created_like = repository_create_like_by_comment_id(&txn, *user_id, *comment_id).await?;

    // 댓글 좋아요 수 증가
    repository_increment_comment_like_count(&txn, comment_id).await?;

    txn.commit().await?;

    // 좋아요 생성 이벤트 로깅
    repository_log_event(
        conn,
        Some(*user_id),
        ActionType::LikeCreated,
        Some(comment.post_id), // 포스트 ID로 로깅
        Some(TargetType::Comment),
        None,
    )
    .await;

    Ok(())
}
