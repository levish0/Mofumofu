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
    pub reason: Option<String>,
}
