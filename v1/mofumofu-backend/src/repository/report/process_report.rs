use crate::entity::common::ReportStatus;
use crate::entity::reports::{
    ActiveModel as ReportActiveModel, Column as ReportColumn, Entity as ReportEntity,
    Model as ReportModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn repository_process_report<C>(
    conn: &C,
    report_id: Uuid,
    status: ReportStatus,
    admin_note: Option<String>,
    resolved_by: Uuid,
) -> Result<ReportModel, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let report = ReportEntity::find()
        .filter(ReportColumn::Id.eq(report_id))
        .one(conn)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(
            "Report not found".to_string(),
        ))?;

    let mut active_report: ReportActiveModel = report.into();
    active_report.status = Set(status);
    active_report.admin_note = Set(admin_note);
    active_report.resolved_by = Set(Some(resolved_by));
    active_report.resolved_at = Set(Some(chrono::Utc::now()));
    active_report.updated_at = Set(Some(chrono::Utc::now()));

    let updated_report = active_report.update(conn).await?;
    Ok(updated_report)
}
