use crate::validator::string_validator::validate_not_blank;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateCommentRequest {
    #[validate(length(
        min = 1,
        max = 10000,
        message = "Content must be between 1 and 10000 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub content: String,
}
