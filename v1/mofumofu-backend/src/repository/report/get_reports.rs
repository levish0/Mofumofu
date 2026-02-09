use crate::entity::common::ReportStatus;
use crate::entity::reports::{Column, Entity as ReportEntity, Model as ReportModel};
use crate::service::error::errors::Errors;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

pub async fn repository_get_reports<C>(
    conn: &C,
    page: u64,
    per_page: u64,
    status_filter: Option<ReportStatus>,
) -> Result<Vec<ReportModel>, Errors>
where
    C: ConnectionTrait,
{
    let offset = (page - 1) * per_page;

    let mut query = ReportEntity::find();

    // 상태 필터 적용
    if let Some(status) = status_filter {
        query = query.filter(Column::Status.eq(status));
    }

    let reports = query
        .order_by_desc(Column::CreatedAt)
        .offset(offset)
        .limit(per_page)
        .all(conn)
        .await?;

    Ok(reports)
}

pub async fn repository_get_reports_count<C>(
    conn: &C,
    status_filter: Option<ReportStatus>,
) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let mut query = ReportEntity::find();

    // 상태 필터 적용
    if let Some(status) = status_filter {
        query = query.filter(Column::Status.eq(status));
    }

    let count = query.count(conn).await?;
    Ok(count)
}
