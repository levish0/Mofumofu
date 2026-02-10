use mofumofu_dto::hashtags::{HashtagResponse, TrendingHashtagsResponse, TrendingQuery};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::get_trending::get_trending,
    ),
    components(
        schemas(
            HashtagResponse,
            TrendingHashtagsResponse,
            TrendingQuery,
        )
    ),
    tags(
        (name = "Hashtags", description = "Hashtag endpoints")
    )
)]
pub struct HashtagsApiDoc;
