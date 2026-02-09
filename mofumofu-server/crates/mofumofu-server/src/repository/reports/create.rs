use mofumofu_entity::common::ReportTargetType;
use mofumofu_entity::reports::{ActiveModel as ReportActiveModel, Model as ReportModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_report<C>(
    conn: &C,
    reporter_id: Uuid,
    target_type: ReportTargetType,
    target_id: Uuid,
    reason: String,
    description: Option<String>,
) -> Result<ReportModel, Errors>
where
    C: ConnectionTrait,
{
    let new_report = ReportActiveModel {
        id: Default::default(),
        reporter_id: Set(reporter_id),
        target_type: Set(target_type),
        target_id: Set(target_id),
        reason: Set(reason),
        description: Set(description),
        status: Default::default(),
        resolved_by: Set(None),
        resolved_at: Set(None),
        created_at: Default::default(),
    };

    let report = new_report.insert(conn).await?;
    Ok(report)
}
