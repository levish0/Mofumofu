use crate::search::request::common::SortOrder;
use crate::validator::string_validator::validate_not_blank;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
pub enum PostSortField {
    CreatedAt,
    LikeCount,
    ViewCount,
    CommentCount,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SearchPostsRequest {
    /// Search query for title, summary, author, or hashtags. Empty or omitted returns all posts.
    #[validate(length(max = 200, message = "Query must be at most 200 characters."))]
    #[validate(custom(function = "validate_not_blank"))]
    #[serde(default)]
    pub query: Option<String>,

    /// Filter by author user ID
    pub user_id: Option<Uuid>,

    #[validate(range(min = 1, message = "Page must be greater than 0"))]
    pub page: u32,

    #[validate(range(min = 1, max = 50, message = "Page size must be between 1 and 50"))]
    pub page_size: u32,

    pub sort_by: Option<PostSortField>,
    pub sort_order: Option<SortOrder>,
}
