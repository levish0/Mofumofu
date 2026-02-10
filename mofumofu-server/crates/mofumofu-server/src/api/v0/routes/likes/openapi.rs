use mofumofu_dto::likes::{LikeRequest, LikeStatusResponse};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::create_like::create_like,
        super::delete_like::delete_like,
        super::check_like_status::check_like_status,
    ),
    components(
        schemas(
            LikeRequest,
            LikeStatusResponse,
        )
    ),
    tags(
        (name = "Likes", description = "Like endpoints")
    )
)]
pub struct LikesApiDoc;
