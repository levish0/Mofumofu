use super::super::find_list::ReportFilter;
use mofumofu_entity::reports::{Column as ReportColumn, Entity as ReportEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

pub async fn repository_exists_newer_report<C>(
    conn: &C,
    filter: &ReportFilter,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let mut query = ReportEntity::find().filter(ReportColumn::Id.gt(cursor_id));

    if let Some(reporter_id) = filter.reporter_id {
        query = query.filter(ReportColumn::ReporterId.eq(reporter_id));
    }

    if let Some(ref target_type) = filter.target_type {
        query = query.filter(ReportColumn::TargetType.eq(target_type.clone()));
    }

    if let Some(target_id) = filter.target_id {
        query = query.filter(ReportColumn::TargetId.eq(target_id));
    }

    if let Some(ref status) = filter.status {
        query = query.filter(ReportColumn::Status.eq(status.clone()));
    }

    let count = query.limit(1).count(conn).await?;
    Ok(count > 0)
}
