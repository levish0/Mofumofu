use crate::entity::common::{ReportReason, ReportStatus, ReportTargetType};
use crate::entity::reports::{ActiveModel as ReportActiveModel, Model as ReportModel};
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_report<C>(
    conn: &C,
    user_id: Option<Uuid>,
    target_type: ReportTargetType,
    target_id: Uuid,
    reasons: Vec<ReportReason>,
    description: Option<String>,
) -> Result<ReportModel, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let reasons_json = serde_json::to_value(reasons)
        .map_err(|_| sea_orm::DbErr::Custom("Failed to serialize reasons".to_string()))?;

    let new_report = ReportActiveModel {
        id: Default::default(),
        reporter_id: Set(user_id),
        target_type: Set(target_type),
        target_id: Set(target_id),
        reasons: Set(reasons_json),
        description: Set(description),
        status: Set(ReportStatus::Pending),
        admin_note: Set(None),
        resolved_by: Set(None),
        resolved_at: Set(None),
        created_at: Default::default(),
        updated_at: Set(None),
    };

    let created_report = new_report.insert(conn).await?;
    Ok(created_report)
}
