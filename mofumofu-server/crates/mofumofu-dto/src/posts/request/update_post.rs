use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub summary: Option<Option<String>>,
    pub thumbnail_image: Option<Option<String>>,
    pub hashtags: Option<Vec<String>>,
    pub publish: Option<bool>,
    pub unpublish: Option<bool>,
}
