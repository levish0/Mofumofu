use std::collections::HashMap;

use crate::repository::comments::{
    repository_exists_newer_comment, repository_exists_older_comment,
    repository_find_comments_by_post_id,
};
use crate::repository::user::repository_find_users_by_ids;
use crate::utils::r2_url::build_r2_public_url;
use mofumofu_dto::comments::{
    CommentAuthor, CommentListResponse, CommentResponse, GetCommentsRequest,
};
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

    // Batch load authors
    let user_ids: Vec<_> = comments.iter().map(|c| c.user_id).collect();
    let users = repository_find_users_by_ids(conn, &user_ids).await?;
    let user_map: HashMap<_, _> = users.into_iter().map(|u| (u.id, u)).collect();

    let data: Vec<CommentResponse> = comments
        .into_iter()
        .map(|comment| {
            let author = user_map
                .get(&comment.user_id)
                .map(|u| CommentAuthor {
                    id: u.id,
                    handle: u.handle.clone(),
                    display_name: u.display_name.clone(),
                    profile_image: u.profile_image.as_deref().map(build_r2_public_url),
                })
                .unwrap_or_else(|| CommentAuthor {
                    id: comment.user_id,
                    handle: String::new(),
                    display_name: String::new(),
                    profile_image: None,
                });

            CommentResponse {
                id: comment.id,
                post_id: comment.post_id,
                user_id: comment.user_id,
                author,
                parent_id: comment.parent_id,
                depth: comment.depth,
                content: comment.content,
                like_count: comment.like_count,
                deleted_at: comment.deleted_at,
                created_at: comment.created_at,
                updated_at: comment.updated_at,
            }
        })
        .collect();

    Ok(CommentListResponse {
        data,
        has_newer,
        has_older,
    })
}
