use crate::dto::report::request::create_report::CreateReportRequest;
use crate::dto::report::response::create_report::CreateReportResponse;
use crate::repository::report::create_report::repository_create_report;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_create_report<C>(
    conn: &C,
    user_id: Option<Uuid>,
    request: CreateReportRequest,
) -> ServiceResult<CreateReportResponse>
where
    C: ConnectionTrait,
{
    let created_report = repository_create_report(
        conn,
        user_id, // 로그인한 사용자의 ID 또는 None (익명)
        request.target_type,
        request.target_id,
        request.reasons,
        request.description,
    )
    .await?;

    Ok(CreateReportResponse {
        report_id: created_report.id,
    })
}
