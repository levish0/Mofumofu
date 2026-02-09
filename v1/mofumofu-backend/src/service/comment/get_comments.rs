use crate::dto::comment::request::GetCommentsRequest;
use crate::dto::comment::response::{CommentInfo, GetCommentsResponse};
use crate::repository::comment::get_comments::{
    repository_count_comments, repository_get_comments,
};
use crate::repository::comment::get_reply_count::repository_get_reply_count;
use crate::repository::like::check_like_status::repository_check_like_status_by_comment_id;
use crate::repository::like::get_like_count::repository_get_like_count_by_comment_id;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::ServiceResult;
use sea_orm::{ConnectionTrait, TransactionTrait};
use uuid::Uuid;

pub async fn service_get_comments<C>(
    conn: &C,
    request: GetCommentsRequest,
) -> ServiceResult<GetCommentsResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    let page = request.page;
    let per_page = request.per_page;

    // 댓글 조회
    let comments =
        repository_get_comments(conn, request.post_id, page, per_page, request.sort.clone())
            .await?;
    let total_count = repository_count_comments(conn, request.post_id).await?;

    let mut comment_infos = Vec::new();

    for comment in comments {
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
                .ok_or(crate::service::error::errors::Errors::UserNotFound)?;

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

        comment_infos.push(comment_info);
    }

    let has_next = comment_infos.len() == per_page as usize;

    Ok(GetCommentsResponse {
        comments: comment_infos,
        total_count,
        page,
        per_page,
        has_next,
    })
}
