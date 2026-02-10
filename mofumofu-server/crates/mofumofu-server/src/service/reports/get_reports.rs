use crate::repository::reports::{
    ReportFilter, repository_exists_newer_report, repository_exists_older_report,
    repository_find_reports,
};
use mofumofu_dto::pagination::CursorDirection;
use mofumofu_dto::reports::{GetReportsRequest, ReportListResponse, ReportResponse};
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;

pub async fn service_get_reports(
    conn: &DatabaseConnection,
    payload: GetReportsRequest,
) -> ServiceResult<ReportListResponse> {
    let is_newer = payload.cursor_direction == Some(CursorDirection::Newer);

    let filter = ReportFilter {
        reporter_id: payload.reporter_id,
        target_type: payload.target_type,
        status: payload.status,
        ..Default::default()
    };

    let mut reports = repository_find_reports(
        conn,
        &filter,
        payload.cursor_id,
        payload.cursor_direction,
        payload.limit,
    )
    .await?;

    let (has_newer, has_older) = if reports.is_empty() {
        (false, false)
    } else {
        let first_id = reports.first().unwrap().id;
        let last_id = reports.last().unwrap().id;
        if is_newer {
            let has_newer = repository_exists_newer_report(conn, &filter, last_id).await?;
            let has_older = repository_exists_older_report(conn, &filter, first_id).await?;
            (has_newer, has_older)
        } else {
            let has_newer = repository_exists_newer_report(conn, &filter, first_id).await?;
            let has_older = repository_exists_older_report(conn, &filter, last_id).await?;
            (has_newer, has_older)
        }
    };

    if is_newer {
        reports.reverse();
    }

    let data: Vec<ReportResponse> = reports.into_iter().map(ReportResponse::from).collect();

    Ok(ReportListResponse {
        data,
        has_newer,
        has_older,
    })
}
