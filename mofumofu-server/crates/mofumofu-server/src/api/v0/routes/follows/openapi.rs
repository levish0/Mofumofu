use mofumofu_dto::follows::{
    FollowListResponse, FollowRequest, FollowStatusResponse, FollowUserItem, GetFollowersRequest,
    GetFollowingRequest,
};
use mofumofu_dto::pagination::CursorDirection;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::create_follow::create_follow,
        super::delete_follow::delete_follow,
        super::check_follow_status::check_follow_status,
        super::get_followers::get_followers,
        super::get_following::get_following,
    ),
    components(
        schemas(
            FollowRequest,
            FollowStatusResponse,
            GetFollowersRequest,
            GetFollowingRequest,
            FollowUserItem,
            FollowListResponse,
            CursorDirection,
        )
    ),
    tags(
        (name = "Follows", description = "Follow endpoints")
    )
)]
pub struct FollowsApiDoc;
