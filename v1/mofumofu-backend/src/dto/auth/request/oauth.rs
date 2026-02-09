use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct GoogleLoginRequest {
    #[validate(length(min = 1, message = "Authorization code is required"))]
    pub code: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Handle must be between 3 and 20 characters"
    ))]
    pub handle: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct GithubLoginRequest {
    #[validate(length(min = 1, message = "Authorization code is required"))]
    pub code: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Handle must be between 3 and 20 characters"
    ))]
    pub handle: Option<String>,
}
