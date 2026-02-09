use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct GetPostByHandleAndSlugRequest {
    #[validate(length(min = 1, max = 20))]
    pub handle: String,
    #[validate(length(min = 1, max = 80))]
    pub slug: String,
}
