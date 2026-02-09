use crate::dto::report::request::get_reports::GetReportsRequest;
use crate::dto::report::response::get_reports::GetReportsResponse;
use crate::dto::report::response::report_info::ReportInfo;
use crate::entity::common::ReportReason;
use crate::repository::report::get_reports::{
    repository_get_reports, repository_get_reports_count,
};
use crate::service::auth::role_check::require_moderator;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_get_reports<C>(
    conn: &C,
    user_id: Uuid,
    request: GetReportsRequest,
) -> ServiceResult<GetReportsResponse>
where
    C: ConnectionTrait,
{
    // 관리자/모더레이터 권한 체크
    require_moderator(conn, user_id).await?;
    let status = request.status.clone();

    let reports =
        repository_get_reports(conn, request.page, request.per_page, status.clone()).await?;

    let total = repository_get_reports_count(conn, status).await?;

    let report_infos: Result<Vec<ReportInfo>, Errors> = reports
        .into_iter()
        .map(|report| {
            let reasons: Vec<ReportReason> =
                serde_json::from_value(report.reasons).map_err(|_| {
                    Errors::DatabaseError("Failed to deserialize report reasons".to_string())
                })?;

            Ok(ReportInfo {
                id: report.id,
                reporter_id: report.reporter_id,
                target_type: report.target_type,
                target_id: report.target_id,
                reasons,
                description: report.description,
                status: report.status,
                created_at: report.created_at,
            })
        })
        .collect();

    let report_infos = report_infos?;

    Ok(GetReportsResponse {
        reports: report_infos,
        total,
        page: request.page,
        per_page: request.per_page,
    })
}
