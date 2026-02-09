use serde::Deserialize;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreatePostRequest {
    #[validate(length(max = 80, message = "Title must be at most 80 characters."))]
    pub title: String,
    #[validate(length(max = 500, message = "Summary must be at most 500 characters."))]
    pub summary: Option<String>,
    #[validate(length(max = 40000, message = "Content must be at most 40000 characters."))]
    pub content: String,
    #[validate(length(max = 80, message = "Slug must be at most 80 characters."))]
    pub slug: String,
    #[validate(custom(function = "validate_hashtags"))]
    pub hashtags: Option<Vec<String>>,
}

fn validate_hashtags(hashtags: &Vec<String>) -> Result<(), ValidationError> {
    if hashtags.len() > 8 {
        let mut error = ValidationError::new("too_many");
        error.message = Some("Maximum 8 hashtags allowed.".into());
        return Err(error);
    }
    for tag in hashtags {
        if tag.len() > 50 {
            let mut error = ValidationError::new("too_long");
            error.message = Some("Each hashtag must be at most 50 characters.".into());
            return Err(error);
        }
    }
    Ok(())
}
