use crate::entity::common::{ReportReason, ReportStatus, ReportTargetType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ReportInfo {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporter_id: Option<Uuid>,
    pub target_type: ReportTargetType,
    pub target_id: Uuid,
    pub reasons: Vec<ReportReason>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub status: ReportStatus,
    pub created_at: DateTime<Utc>,
}
