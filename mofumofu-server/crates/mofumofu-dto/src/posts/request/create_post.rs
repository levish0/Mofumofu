use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePostRequest {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub summary: Option<String>,
    pub thumbnail_image: Option<String>,
    pub hashtags: Option<Vec<String>>,
    pub publish: Option<bool>,
}
