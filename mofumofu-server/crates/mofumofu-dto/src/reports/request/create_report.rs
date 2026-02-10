use crate::validator::string_validator::validate_not_blank;
use mofumofu_entity::common::ReportTargetType;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateReportRequest {
    pub target_type: ReportTargetType,
    pub target_id: Uuid,
    #[validate(length(
        min = 1,
        max = 500,
        message = "Reason must be between 1 and 500 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub reason: String,
    #[validate(length(max = 2000, message = "Description must be at most 2000 characters."))]
    #[validate(custom(function = "validate_not_blank"))]
    pub description: Option<String>,
}
