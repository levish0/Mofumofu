use mofumofu_dto::follows::{FollowRequest, FollowStatusResponse};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::create_follow::create_follow,
        super::delete_follow::delete_follow,
        super::check_follow_status::check_follow_status,
    ),
    components(
        schemas(
            FollowRequest,
            FollowStatusResponse,
        )
    ),
    tags(
        (name = "Follows", description = "Follow endpoints")
    )
)]
pub struct FollowsApiDoc;
