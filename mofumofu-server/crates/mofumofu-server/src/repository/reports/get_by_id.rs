use mofumofu_entity::reports::{Entity as ReportEntity, Model as ReportModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait};
use uuid::Uuid;

pub async fn repository_get_report_by_id<C>(conn: &C, id: Uuid) -> Result<ReportModel, Errors>
where
    C: ConnectionTrait,
{
    let report = ReportEntity::find_by_id(id).one(conn).await?;
    report.ok_or(Errors::ReportNotFound)
}
