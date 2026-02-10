use crate::validator::custom::hashtag_validator::validate_hashtags;
use crate::validator::string_validator::validate_not_blank;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePostRequest {
    #[validate(length(
        min = 1,
        max = 150,
        message = "Title must be between 1 and 150 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub title: Option<String>,
    #[validate(length(
        min = 1,
        max = 200,
        message = "Slug must be between 1 and 200 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub slug: Option<String>,
    #[validate(length(
        min = 1,
        max = 100000,
        message = "Content must be between 1 and 100000 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub content: Option<String>,
    #[validate(length(max = 500, message = "Summary cannot exceed 500 characters."))]
    pub summary: Option<Option<String>>,
    pub thumbnail_image: Option<Option<String>>,
    #[validate(length(max = 30, message = "Cannot have more than 30 hashtags."))]
    #[validate(custom(function = "validate_hashtags"))]
    pub hashtags: Option<Vec<String>>,
    pub publish: Option<bool>,
    pub unpublish: Option<bool>,
}
