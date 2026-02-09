use crate::entity::common::{ReportReason, ReportTargetType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateReportRequest {
    pub target_type: ReportTargetType,
    pub target_id: Uuid,
    pub reasons: Vec<ReportReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
