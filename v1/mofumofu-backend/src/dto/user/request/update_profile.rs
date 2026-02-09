use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, Debug, ToSchema)]
pub struct UpdateProfileRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Name must be between 3 and 20 characters."
    ))]
    pub name: Option<String>,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Handle must be between 3 and 20 characters."
    ))]
    pub handle: Option<String>,
    #[validate(length(max = 200))]
    pub bio: Option<String>,
    #[validate(length(max = 30))]
    pub location: Option<String>,
    #[validate(length(max = 50))]
    pub website: Option<String>,
    #[validate(length(
        min = 6,
        max = 20,
        message = "Password must be between 6 and 20 characters."
    ))]
    pub password: Option<String>,
    #[validate(length(
        min = 6,
        max = 20,
        message = "Current password must be between 6 and 20 characters."
    ))]
    pub current_password: Option<String>,
}
