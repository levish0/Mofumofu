use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::report::request::get_reports::GetReportsRequest;
use crate::dto::report::response::get_reports::GetReportsResponse;
use crate::service::error::errors::Errors;
use crate::service::report::get_reports::service_get_reports;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/report/list",
    request_body = GetReportsRequest,
    responses(
        (status = 200, description = "Reports retrieved successfully", body = GetReportsResponse),
        (status = StatusCode::FORBIDDEN, description = "Access denied - Admin/Moderator required"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Report"
)]
pub async fn get_reports(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<GetReportsRequest>,
) -> Result<GetReportsResponse, Errors> {
    info!("Received request to get reports: {:?}", payload);

    let user_id = claims.sub.clone();
    let response = service_get_reports(&state.conn, user_id, payload).await?;

    Ok(response)
}
