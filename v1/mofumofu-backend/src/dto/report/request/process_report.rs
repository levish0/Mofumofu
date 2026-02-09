use crate::entity::common::ReportStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct ProcessReportRequest {
    pub report_id: Uuid,
    pub status: ReportStatus, // Resolved, Dismissed 등
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_note: Option<String>,
}
