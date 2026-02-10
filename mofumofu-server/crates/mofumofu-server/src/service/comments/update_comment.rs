use crate::repository::comments::{
    CommentUpdateParams, repository_get_comment_by_id, repository_update_comment,
};
use crate::repository::user::repository_get_user_by_id;
use crate::utils::r2_url::build_r2_public_url;
use mofumofu_dto::comments::{CommentAuthor, CommentResponse, UpdateCommentRequest};
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

    let user = repository_get_user_by_id(conn, user_id).await?;
    let author = CommentAuthor {
        id: user.id,
        handle: user.handle,
        display_name: user.display_name,
        profile_image: user.profile_image.as_deref().map(build_r2_public_url),
    };

    Ok(CommentResponse {
        id: updated.id,
        post_id: updated.post_id,
        user_id: updated.user_id,
        author,
        parent_id: updated.parent_id,
        depth: updated.depth,
        content: updated.content,
        like_count: updated.like_count,
        deleted_at: updated.deleted_at,
        created_at: updated.created_at,
        updated_at: updated.updated_at,
    })
}
