use crate::extractors::RequiredSession;
use crate::service::reports::service_update_report;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use mofumofu_dto::reports::{ReportIdPath, ReportResponse, UpdateReportRequest};
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    patch,
    path = "/v0/reports/{report_id}",
    params(ReportIdPath),
    request_body = UpdateReportRequest,
    responses(
        (status = 200, description = "Report updated successfully", body = ReportResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 404, description = "Not Found - Report does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Reports"
)]
pub async fn update_report(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<ReportIdPath>,
    ValidatedJson(payload): ValidatedJson<UpdateReportRequest>,
) -> Result<Json<ReportResponse>, Errors> {
    let response = service_update_report(
        &state.write_db,
        session_context.user_id,
        path.report_id,
        payload,
    )
    .await?;
    Ok(Json(response))
}
