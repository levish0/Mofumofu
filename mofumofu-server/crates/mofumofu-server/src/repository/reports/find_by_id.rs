use mofumofu_entity::reports::{Entity as ReportEntity, Model as ReportModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait};
use uuid::Uuid;

pub async fn repository_find_report_by_id<C>(
    conn: &C,
    id: Uuid,
) -> Result<Option<ReportModel>, Errors>
where
    C: ConnectionTrait,
{
    let report = ReportEntity::find_by_id(id).one(conn).await?;
    Ok(report)
}
