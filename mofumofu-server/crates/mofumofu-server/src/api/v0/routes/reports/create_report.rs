use crate::extractors::RequiredSession;
use crate::service::reports::service_create_report;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mofumofu_dto::reports::CreateReportRequest;
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/reports",
    request_body = CreateReportRequest,
    responses(
        (status = 201, description = "Report created successfully", body = mofumofu_dto::reports::ReportResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Reports"
)]
pub async fn create_report(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<CreateReportRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_create_report(&state.write_db, session_context.user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
