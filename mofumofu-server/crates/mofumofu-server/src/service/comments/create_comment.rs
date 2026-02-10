use crate::repository::comments::{repository_create_comment, repository_get_comment_by_id};
use crate::repository::posts::{
    repository_get_post_by_id, repository_increment_post_comment_count,
};
use mofumofu_dto::comments::{CommentResponse, CreateCommentRequest};
use mofumofu_errors::errors::{Errors, ServiceResult};
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

pub async fn service_create_comment(
    conn: &DatabaseConnection,
    user_id: Uuid,
    payload: CreateCommentRequest,
) -> ServiceResult<CommentResponse> {
    let txn = conn.begin().await?;

    repository_get_post_by_id(&txn, payload.post_id).await?;

    let depth = if let Some(parent_id) = payload.parent_id {
        let parent = repository_get_comment_by_id(&txn, parent_id)
            .await
            .map_err(|_| Errors::CommentParentNotFound)?;

        if parent.post_id != payload.post_id {
            return Err(Errors::CommentPostMismatch);
        }

        parent.depth + 1
    } else {
        0
    };

    let comment = repository_create_comment(
        &txn,
        payload.post_id,
        user_id,
        payload.parent_id,
        depth,
        payload.content,
    )
    .await?;

    repository_increment_post_comment_count(&txn, payload.post_id).await?;

    txn.commit().await?;

    Ok(CommentResponse::from(comment))
}
