use crate::dto::comment::request::DeleteCommentRequest;
use crate::repository::comment::delete_comment::repository_delete_comment;
use crate::repository::comment::get_comment_by_id::repository_get_comment_by_id;
use crate::repository::comment::update_reply_count::repository_decrement_reply_count;
use crate::repository::post::update_comment_count::repository_decrement_comment_count;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use uuid::Uuid;

pub async fn service_delete_comment<C>(
    conn: &C,
    user_id: &Uuid,
    request: DeleteCommentRequest,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 댓글 존재 확인 및 권한 체크
    let comment = repository_get_comment_by_id(&txn, request.comment_id)
        .await?
        .ok_or(Errors::CommentNotFound)?;

    // 작성자만 삭제 가능 (또는 관리자 권한 체크 추가 가능)
    if comment.user_id != *user_id {
        return Err(Errors::UserUnauthorized);
    }

    // 이미 삭제된 댓글인지 확인
    if comment.is_deleted {
        return Err(Errors::CommentNotFound);
    }

    // 댓글 soft delete
    repository_delete_comment(&txn, request.comment_id).await?;

    // 포스트 댓글 수 감소
    repository_decrement_comment_count(&txn, &comment.post_id).await?;

    // 부모 댓글이 있다면 답글 수 감소
    if let Some(parent_id) = comment.parent_id {
        repository_decrement_reply_count(&txn, &parent_id).await?;
    }

    txn.commit().await?;
    Ok(())
}
