use crate::pagination::CursorDirection;
use mofumofu_entity::common::{ReportStatus, ReportTargetType};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetReportsRequest {
    pub cursor_id: Option<Uuid>,
    pub cursor_direction: Option<CursorDirection>,
    #[validate(range(min = 1, max = 100, message = "Limit must be between 1 and 100."))]
    pub limit: u64,
    pub reporter_id: Option<Uuid>,
    pub target_type: Option<ReportTargetType>,
    pub status: Option<ReportStatus>,
}
