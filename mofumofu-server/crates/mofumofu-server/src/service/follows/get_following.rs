use crate::repository::follows::{
    repository_exists_newer_following, repository_exists_older_following, repository_find_following,
};
use crate::repository::user::repository_find_users_by_ids;
use crate::utils::r2_url::build_r2_public_url;
use mofumofu_dto::follows::{FollowListResponse, FollowUserItem, GetFollowingRequest};
use mofumofu_dto::pagination::CursorDirection;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

pub async fn service_get_following(
    conn: &DatabaseConnection,
    payload: GetFollowingRequest,
) -> ServiceResult<FollowListResponse> {
    let is_newer = payload.cursor_direction == Some(CursorDirection::Newer);

    let mut follows = repository_find_following(
        conn,
        payload.user_id,
        payload.cursor_id,
        payload.cursor_direction,
        payload.limit,
    )
    .await?;

    let (has_newer, has_older) = if follows.is_empty() {
        (false, false)
    } else {
        let first_id = follows.first().unwrap().id;
        let last_id = follows.last().unwrap().id;
        if is_newer {
            let has_newer =
                repository_exists_newer_following(conn, payload.user_id, last_id).await?;
            let has_older =
                repository_exists_older_following(conn, payload.user_id, first_id).await?;
            (has_newer, has_older)
        } else {
            let has_newer =
                repository_exists_newer_following(conn, payload.user_id, first_id).await?;
            let has_older =
                repository_exists_older_following(conn, payload.user_id, last_id).await?;
            (has_newer, has_older)
        }
    };

    if is_newer {
        follows.reverse();
    }

    let user_ids: Vec<_> = follows.iter().map(|f| f.followee_id).collect();
    let users = repository_find_users_by_ids(conn, &user_ids).await?;
    let user_map: HashMap<_, _> = users.into_iter().map(|u| (u.id, u)).collect();

    let data = follows
        .into_iter()
        .filter_map(|f| {
            let user = user_map.get(&f.followee_id)?;
            Some(FollowUserItem {
                id: user.id,
                handle: user.handle.clone(),
                display_name: user.display_name.clone(),
                profile_image: user.profile_image.as_deref().map(build_r2_public_url),
                followed_at: f.created_at,
            })
        })
        .collect();

    Ok(FollowListResponse {
        data,
        has_newer,
        has_older,
    })
}
