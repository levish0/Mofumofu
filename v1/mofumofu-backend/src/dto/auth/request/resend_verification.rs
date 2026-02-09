use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct ResendVerificationRequest {
    #[validate(email(message = "Invalid email format."))]
    pub email: String,
}
