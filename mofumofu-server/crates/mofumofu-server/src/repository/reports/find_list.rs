use mofumofu_dto::pagination::CursorDirection;
use mofumofu_entity::common::{ReportStatus, ReportTargetType};
use mofumofu_entity::reports::{
    Column as ReportColumn, Entity as ReportEntity, Model as ReportModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct ReportFilter {
    pub reporter_id: Option<Uuid>,
    pub target_type: Option<ReportTargetType>,
    pub target_id: Option<Uuid>,
    pub status: Option<ReportStatus>,
}

pub async fn repository_find_reports<C>(
    conn: &C,
    filter: &ReportFilter,
    cursor_id: Option<Uuid>,
    cursor_direction: Option<CursorDirection>,
    limit: u64,
) -> Result<Vec<ReportModel>, Errors>
where
    C: ConnectionTrait,
{
    let mut query = ReportEntity::find();

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

    if let Some(id) = cursor_id {
        let direction = cursor_direction.unwrap_or(CursorDirection::Older);
        query = match direction {
            CursorDirection::Older => query
                .filter(ReportColumn::Id.lt(id))
                .order_by_desc(ReportColumn::Id),
            CursorDirection::Newer => query
                .filter(ReportColumn::Id.gt(id))
                .order_by_asc(ReportColumn::Id),
        };
    } else {
        query = query.order_by_desc(ReportColumn::Id);
    }

    let reports = query.limit(limit).all(conn).await?;

    Ok(reports)
}
