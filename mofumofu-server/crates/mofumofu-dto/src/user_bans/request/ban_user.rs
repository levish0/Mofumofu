use crate::validator::string_validator::validate_not_blank;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BanUserRequest {
    pub user_id: Uuid,
    #[validate(length(
        min = 1,
        max = 1000,
        message = "Reason must be between 1 and 1000 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub reason: String,
    pub expires_at: Option<DateTime<Utc>>,
}
