use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TrendingHashtagsRequest {
    #[validate(range(min = 1, max = 30, message = "days must be between 1 and 30"))]
    pub days: Option<i64>,

    #[validate(range(min = 1, max = 50, message = "limit must be between 1 and 50"))]
    pub limit: Option<u64>,
}

impl Default for TrendingHashtagsRequest {
    fn default() -> Self {
        Self {
            days: Some(7),
            limit: Some(16),
        }
    }
}
