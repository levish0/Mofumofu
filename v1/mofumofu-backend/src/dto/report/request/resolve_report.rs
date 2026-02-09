use crate::entity::common::ReportStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResolveReportRequest {
    pub report_id: Uuid,
    pub status: ReportStatus, // resolved 또는 dismissed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_note: Option<String>,
}
