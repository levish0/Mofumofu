use crate::dto::report::request::process_report::ProcessReportRequest;
use crate::repository::report::process_report::repository_process_report;
use crate::service::auth::role_check::require_moderator;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_process_report<C>(
    conn: &C,
    user_id: Uuid,
    request: ProcessReportRequest,
) -> ServiceResult<()>
where
    C: ConnectionTrait,
{
    // 관리자/모더레이터 권한 체크
    require_moderator(conn, user_id).await?;

    let _updated_report = repository_process_report(
        conn,
        request.report_id,
        request.status,
        request.admin_note,
        user_id,
    )
    .await?;

    Ok(())
}
