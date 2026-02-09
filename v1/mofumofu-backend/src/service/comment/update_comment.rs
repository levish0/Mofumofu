use crate::dto::comment::request::UpdateCommentRequest;
use crate::repository::comment::get_comment_by_id::repository_get_comment_by_id;
use crate::repository::comment::update_comment::repository_update_comment;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use uuid::Uuid;

pub async fn service_update_comment<C>(
    conn: &C,
    user_id: &Uuid,
    request: UpdateCommentRequest,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 댓글 존재 확인 및 권한 체크
    let comment = repository_get_comment_by_id(&txn, request.comment_id)
        .await?
        .ok_or(Errors::CommentNotFound)?;

    // 작성자만 수정 가능
    if comment.user_id != *user_id {
        return Err(Errors::UserUnauthorized);
    }

    // 삭제된 댓글은 수정 불가
    if comment.is_deleted {
        return Err(Errors::CommentNotFound);
    }

    // 댓글 업데이트
    repository_update_comment(&txn, request.comment_id, &request.content).await?;

    txn.commit().await?;
    Ok(())
}
