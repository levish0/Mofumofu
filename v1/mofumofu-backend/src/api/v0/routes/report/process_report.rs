use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::report::request::process_report::ProcessReportRequest;
use crate::service::error::errors::Errors;
use crate::service::report::process_report::service_process_report;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    put,
    path = "/v0/report/process",
    request_body = ProcessReportRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "Report processed successfully"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid request"),
        (status = StatusCode::FORBIDDEN, description = "Access denied - Admin/Moderator required"),
        (status = StatusCode::NOT_FOUND, description = "Report not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Report"
)]
pub async fn process_report(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<ProcessReportRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received request to process report: {:?}", payload);
    let user_id = claims.sub.clone();

    service_process_report(&state.conn, user_id, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
