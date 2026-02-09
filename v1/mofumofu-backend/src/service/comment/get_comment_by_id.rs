use crate::dto::comment::response::CommentInfo;
use crate::repository::comment::get_comment_by_id::repository_get_comment_by_id;
use crate::repository::comment::get_reply_count::repository_get_reply_count;
use crate::repository::like::check_like_status::repository_check_like_status_by_comment_id;
use crate::repository::like::get_like_count::repository_get_like_count_by_comment_id;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionTrait};
use uuid::Uuid;

pub async fn service_get_comment_by_id<C>(conn: &C, comment_id: Uuid) -> ServiceResult<CommentInfo>
where
    C: ConnectionTrait + TransactionTrait,
{
    // 댓글 조회
    let comment = repository_get_comment_by_id(conn, comment_id)
        .await?
        .ok_or(Errors::CommentNotFound)?;

    let like_count = repository_get_like_count_by_comment_id(conn, comment.id).await? as i32;
    let reply_count = repository_get_reply_count(conn, comment.id).await? as i32;

    // 삭제된 댓글은 내용과 사용자 정보를 숨김
    let comment_info = if comment.is_deleted {
        CommentInfo {
            id: comment.id,
            content: None,
            post_id: comment.post_id,
            user_id: None,
            user_handle: None,
            user_name: None,
            user_profile_image: None,
            parent_id: comment.parent_id,
            like_count,
            reply_count,
            is_deleted: comment.is_deleted,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        }
    } else {
        let user = repository_find_user_by_uuid(conn, &comment.user_id)
            .await?
            .ok_or(Errors::UserNotFound)?;

        CommentInfo {
            id: comment.id,
            content: Some(comment.content),
            post_id: comment.post_id,
            user_id: Some(comment.user_id),
            user_handle: Some(user.handle),
            user_name: Some(user.name),
            user_profile_image: user.profile_image,
            parent_id: comment.parent_id,
            like_count,
            reply_count,
            is_deleted: comment.is_deleted,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        }
    };

    Ok(comment_info)
}
