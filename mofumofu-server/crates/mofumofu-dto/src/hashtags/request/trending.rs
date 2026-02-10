use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct TrendingQuery {
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<u64>,
}
