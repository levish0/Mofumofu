use crate::dto::common::PaginationQuery;
use crate::dto::follow::response::follow_list::FollowListResponse;
use crate::dto::user::response::info::UserInfoResponse;
use crate::service::error::errors::Errors;
use crate::service::follow::get_follower_count::service_get_follower_count;
use crate::service::follow::get_followers_list::service_get_followers;
use crate::state::AppState;
use axum::extract::{Path, Query, State};

// 팔로워 목록 조회
#[utoipa::path(
    get,
    path = "/v0/followers/{handle}",
    params(
        ("handle" = String, Path, description = "User handle"),
        ("page" = Option<u64>, Query, description = "Page number (default: 1)"),
        ("per_page" = Option<u64>, Query, description = "Items per page (default: 20)")
    ),
    responses(
        (status = 200, description = "Followers list", body = FollowListResponse),
        (status = 404, description = "User not found"),
    ),
    tag = "Follow"
)]
pub async fn get_followers(
    State(state): State<AppState>,
    Path(handle): Path<String>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<FollowListResponse, Errors> {
    let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;

    let mut users =
        service_get_followers(&state.conn, &handle, offset, pagination.per_page + 1).await?;

    let has_more = users.len() > pagination.per_page as usize;
    if has_more {
        users.pop(); // Remove extra item used for has_more check
    }

    let user_responses: Vec<UserInfoResponse> = users
        .into_iter()
        .map(|user| UserInfoResponse {
            name: user.name,
            handle: user.handle,
            email: user.email,
            bio: user.bio,
            location: user.location,
            website: user.website,
            profile_image: user.profile_image,
            banner_image: user.banner_image,
            is_verified: user.is_verified,
            created_at: user.created_at,
        })
        .collect();

    // Get total count for pagination info
    let total_count = service_get_follower_count(&state.conn, &handle).await?;

    Ok(FollowListResponse {
        users: user_responses,
        total_count,
        page: pagination.page,
        per_page: pagination.per_page,
        has_more,
    })
}
