use crate::extractors::RequiredSession;
use crate::service::reports::service_get_reports;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::reports::{GetReportsRequest, ReportListResponse};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/reports",
    params(GetReportsRequest),
    responses(
        (status = 200, description = "Reports retrieved successfully", body = ReportListResponse),
        (status = 400, description = "Bad request - Invalid query parameters"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Reports"
)]
pub async fn get_reports(
    State(state): State<AppState>,
    RequiredSession(_session_context): RequiredSession,
    ValidatedQuery(payload): ValidatedQuery<GetReportsRequest>,
) -> Result<ReportListResponse, Errors> {
    service_get_reports(&state.read_db, payload).await
}
