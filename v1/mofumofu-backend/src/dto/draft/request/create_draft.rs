use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreateDraftRequest {
    #[validate(length(max = 80, message = "Title must be at most 80 characters."))]
    pub title: Option<String>,
    pub thumbnail_image: Option<String>,
    #[validate(length(max = 500, message = "Summary must be at most 500 characters."))]
    pub summary: Option<String>,
    #[validate(length(max = 40000, message = "Content must be at most 40000 characters."))]
    pub content: Option<String>,
    #[validate(length(
        min = 1,
        max = 80,
        message = "Slug must be between 1 and 80 characters."
    ))]
    pub slug: String,
}
