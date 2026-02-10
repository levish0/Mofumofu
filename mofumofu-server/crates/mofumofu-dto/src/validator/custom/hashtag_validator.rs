use validator::ValidationError;

pub fn validate_hashtags(tags: &[String]) -> Result<(), ValidationError> {
    for tag in tags {
        if tag.is_empty() || tag.len() > 50 {
            return Err(ValidationError::new(
                "Each hashtag must be between 1 and 50 characters.",
            ));
        }
    }
    Ok(())
}
