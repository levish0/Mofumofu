use mofumofu_entity::common::ReportStatus;
use mofumofu_entity::reports::{
    ActiveModel as ReportActiveModel, Entity as ReportEntity, Model as ReportModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel, Set};
use uuid::Uuid;

#[derive(Default)]
pub struct ReportUpdateParams {
    pub status: Option<ReportStatus>,
    pub resolved_by: Option<Option<Uuid>>,
    pub resolved_at: Option<Option<DateTimeUtc>>,
}

pub async fn repository_update_report<C>(
    conn: &C,
    report_id: Uuid,
    params: ReportUpdateParams,
) -> Result<ReportModel, Errors>
where
    C: ConnectionTrait,
{
    let report = ReportEntity::find_by_id(report_id)
        .one(conn)
        .await?
        .ok_or(Errors::ReportNotFound)?;

    let mut report_active: ReportActiveModel = report.into_active_model();

    if let Some(status) = params.status {
        report_active.status = Set(status);
    }
    if let Some(resolved_by) = params.resolved_by {
        report_active.resolved_by = Set(resolved_by);
    }
    if let Some(resolved_at) = params.resolved_at {
        report_active.resolved_at = Set(resolved_at);
    }

    let updated_report = report_active.update(conn).await?;
    Ok(updated_report)
}
