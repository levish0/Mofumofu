use crate::validator::string_validator::validate_not_blank;
use mofumofu_entity::common::ReportStatus;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateReportRequest {
    pub status: Option<ReportStatus>,
    #[validate(length(
        min = 1,
        max = 500,
        message = "Reason must be between 1 and 500 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub reason: Option<String>,
}
