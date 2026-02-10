use crate::repository::comments::{
    CommentUpdateParams, repository_get_comment_by_id, repository_update_comment,
};
use mofumofu_dto::comments::{CommentResponse, UpdateCommentRequest};
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_update_comment(
    conn: &DatabaseConnection,
    user_id: Uuid,
    comment_id: Uuid,
    payload: UpdateCommentRequest,
) -> ServiceResult<CommentResponse> {
    let txn = conn.begin().await?;

    let comment = repository_get_comment_by_id(&txn, comment_id).await?;

    if comment.user_id != user_id {
        return Err(Errors::ForbiddenError(
            "Not the owner of this resource".to_string(),
        ));
    }

    if comment.deleted_at.is_some() {
        return Err(Errors::CommentNotFound);
    }

    let params = CommentUpdateParams {
        content: Some(payload.content),
    };

    let updated = repository_update_comment(&txn, comment_id, params).await?;

    txn.commit().await?;

    Ok(CommentResponse::from(updated))
}
