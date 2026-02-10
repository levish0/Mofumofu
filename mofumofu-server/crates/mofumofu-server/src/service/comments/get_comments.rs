use crate::repository::comments::{
    repository_exists_newer_comment, repository_exists_older_comment,
    repository_find_comments_by_post_id,
};
use mofumofu_dto::comments::{CommentListResponse, CommentResponse, GetCommentsRequest};
use mofumofu_dto::pagination::CursorDirection;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;

pub async fn service_get_comments(
    conn: &DatabaseConnection,
    payload: GetCommentsRequest,
) -> ServiceResult<CommentListResponse> {
    let is_newer = payload.cursor_direction == Some(CursorDirection::Newer);

    let mut comments = repository_find_comments_by_post_id(
        conn,
        payload.post_id,
        payload.cursor_id,
        payload.cursor_direction,
        payload.limit,
    )
    .await?;

    let (has_newer, has_older) = if comments.is_empty() {
        (false, false)
    } else {
        let first_id = comments.first().unwrap().id;
        let last_id = comments.last().unwrap().id;
        if is_newer {
            let has_newer = repository_exists_newer_comment(conn, payload.post_id, last_id).await?;
            let has_older =
                repository_exists_older_comment(conn, payload.post_id, first_id).await?;
            (has_newer, has_older)
        } else {
            let has_newer =
                repository_exists_newer_comment(conn, payload.post_id, first_id).await?;
            let has_older = repository_exists_older_comment(conn, payload.post_id, last_id).await?;
            (has_newer, has_older)
        }
    };

    if is_newer {
        comments.reverse();
    }

    let data: Vec<CommentResponse> = comments.into_iter().map(CommentResponse::from).collect();

    Ok(CommentListResponse {
        data,
        has_newer,
        has_older,
    })
}
