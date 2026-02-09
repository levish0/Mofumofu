use crate::dto::comment::request::CreateCommentRequest;
use crate::dto::comment::response::CreateCommentResponse;
use crate::repository::comment::create_comment::repository_create_comment;
use crate::repository::comment::update_reply_count::repository_increment_reply_count;
use crate::repository::post::get_post_by_uuid::repository_get_post_by_uuid;
use crate::repository::post::update_comment_count::repository_increment_comment_count;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use uuid::Uuid;

pub async fn service_create_comment<C>(
    conn: &C,
    user_id: &Uuid,
    request: CreateCommentRequest,
) -> ServiceResult<CreateCommentResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 포스트 존재 확인
    let _post = repository_get_post_by_uuid(&txn, &request.post_id).await?;

    // 부모 댓글이 있다면 존재 확인
    if let Some(parent_id) = request.parent_id {
        let parent_comment =
            crate::repository::comment::get_comment_by_id::repository_get_comment_by_id(
                &txn, parent_id,
            )
            .await?
            .ok_or(Errors::CommentNotFound)?;

        // 부모 댓글이 같은 포스트에 속하는지 확인
        if parent_comment.post_id != request.post_id {
            return Err(Errors::InvalidParentComment);
        }

        // 삭제된 댓글에는 답글을 달 수 없음
        if parent_comment.is_deleted {
            return Err(Errors::CannotReplyToDeletedComment);
        }
    }

    // 댓글 생성
    let created_comment = repository_create_comment(
        &txn,
        *user_id,
        request.post_id,
        &request.content,
        request.parent_id,
    )
    .await?;

    // 포스트 댓글 수 증가
    repository_increment_comment_count(&txn, &request.post_id).await?;

    // 부모 댓글이 있다면 답글 수 증가
    if let Some(parent_id) = request.parent_id {
        repository_increment_reply_count(&txn, &parent_id).await?;
    }

    txn.commit().await?;
    Ok(CreateCommentResponse {
        comment_id: created_comment.id,
    })
}
