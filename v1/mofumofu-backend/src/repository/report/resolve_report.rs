use crate::entity::common::ReportStatus;
use crate::entity::reports::{ActiveModel as ReportActiveModel, Column, Entity as ReportEntity};
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn repository_resolve_report<C>(
    conn: &C,
    report_id: Uuid,
    status: ReportStatus,
    admin_note: Option<String>,
    resolved_by: Uuid,
) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    let report = ReportEntity::find()
        .filter(Column::Id.eq(report_id))
        .one(conn)
        .await?
        .ok_or(Errors::ReportNotFound)?;

    let mut active_report: ReportActiveModel = report.into();
    active_report.status = Set(status);
    active_report.admin_note = Set(admin_note);
    active_report.resolved_by = Set(Some(resolved_by));
    active_report.resolved_at = Set(Some(Utc::now()));
    active_report.updated_at = Set(Some(Utc::now()));

    active_report.update(conn).await?;
    Ok(())
}
