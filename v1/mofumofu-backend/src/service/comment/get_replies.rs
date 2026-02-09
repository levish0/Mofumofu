use crate::dto::comment::request::GetRepliesRequest;
use crate::dto::comment::response::{CommentInfo, GetRepliesResponse};
use crate::repository::comment::get_comments::{repository_count_replies, repository_get_replies};
use crate::repository::comment::get_reply_count::repository_get_reply_count;
use crate::repository::like::check_like_status::repository_check_like_status_by_comment_id;
use crate::repository::like::get_like_count::repository_get_like_count_by_comment_id;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ConnectionTrait, TransactionTrait};
use uuid::Uuid;

pub async fn service_get_replies<C>(
    conn: &C,
    request: GetRepliesRequest,
) -> ServiceResult<GetRepliesResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    let page = request.page;
    let per_page = request.per_page;

    // 답글 조회
    let replies = repository_get_replies(
        conn,
        request.parent_comment_id,
        page,
        per_page,
        request.sort.clone(),
    )
    .await?;
    let total_count = repository_count_replies(conn, request.parent_comment_id).await?;

    let mut reply_infos = Vec::new();

    for reply in replies {
        let like_count = repository_get_like_count_by_comment_id(conn, reply.id).await? as i32;
        let reply_count = repository_get_reply_count(conn, reply.id).await? as i32;

        // 삭제된 댓글은 내용과 사용자 정보를 숨김
        let reply_info = if reply.is_deleted {
            CommentInfo {
                id: reply.id,
                content: None,
                post_id: reply.post_id,
                user_id: None,
                user_handle: None,
                user_name: None,
                user_profile_image: None,
                parent_id: reply.parent_id,
                like_count,
                reply_count,
                is_deleted: reply.is_deleted,
                created_at: reply.created_at,
                updated_at: reply.updated_at,
            }
        } else {
            let user = repository_find_user_by_uuid(conn, &reply.user_id)
                .await?
                .ok_or(crate::service::error::errors::Errors::UserNotFound)?;

            CommentInfo {
                id: reply.id,
                content: Some(reply.content),
                post_id: reply.post_id,
                user_id: Some(reply.user_id),
                user_handle: Some(user.handle),
                user_name: Some(user.name),
                user_profile_image: user.profile_image,
                parent_id: reply.parent_id,
                like_count,
                reply_count,
                is_deleted: reply.is_deleted,
                created_at: reply.created_at,
                updated_at: reply.updated_at,
            }
        };

        reply_infos.push(reply_info);
    }

    let has_next = reply_infos.len() == per_page as usize;

    Ok(GetRepliesResponse {
        replies: reply_infos,
        total_count,
        page,
        per_page,
        has_next,
    })
}
