use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PublishDraftRequest {
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub thumbnail_image: Option<String>,
    pub hashtags: Option<Vec<String>>,
    pub delete_draft: Option<bool>,
}
