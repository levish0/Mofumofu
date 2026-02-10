use chrono::{DateTime, Utc};
use mofumofu_entity::common::{ReportStatus, ReportTargetType};
use mofumofu_entity::reports::Model as ReportModel;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ReportResponse {
    pub id: Uuid,
    pub reporter_id: Uuid,
    pub target_type: ReportTargetType,
    pub target_id: Uuid,
    pub reason: String,
    pub description: Option<String>,
    pub status: ReportStatus,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<ReportModel> for ReportResponse {
    fn from(model: ReportModel) -> Self {
        Self {
            id: model.id,
            reporter_id: model.reporter_id,
            target_type: model.target_type,
            target_id: model.target_id,
            reason: model.reason,
            description: model.description,
            status: model.status,
            resolved_by: model.resolved_by,
            resolved_at: model.resolved_at,
            created_at: model.created_at,
        }
    }
}
