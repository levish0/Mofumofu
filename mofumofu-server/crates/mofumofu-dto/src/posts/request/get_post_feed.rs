use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Clone, Copy, Deserialize, ToSchema)]
pub enum PostSortOrder {
    Latest,
    Popular,
    Oldest,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetPostFeedRequest {
    pub sort: Option<PostSortOrder>,

    /// Only include posts published after this timestamp (for trending)
    pub published_at_after: Option<DateTime<Utc>>,

    #[validate(range(min = 1, message = "Page must be greater than 0."))]
    pub page: Option<u32>,

    #[validate(range(min = 1, max = 50, message = "Page size must be between 1 and 50."))]
    pub page_size: Option<u32>,
}
