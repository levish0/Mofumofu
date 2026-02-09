use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum PostSortOrder {
    Latest,
    Popular,
    Oldest,
}

impl Default for PostSortOrder {
    fn default() -> Self {
        Self::Latest
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GetPostsRequest {
    #[validate(range(min = 1, message = "Page must be greater than 0."))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 20, message = "Page size must be between 1 and 20."))]
    pub page_size: Option<u32>,

    pub sort: Option<PostSortOrder>,
}

impl Default for GetPostsRequest {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(20),
            sort: Some(PostSortOrder::Latest),
        }
    }
}
