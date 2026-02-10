use crate::extractors::RequiredSession;
use crate::service::moderation_logs::service_get_moderation_logs;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::moderation_logs::{GetModerationLogsRequest, ModerationLogListResponse};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/moderation-logs",
    params(GetModerationLogsRequest),
    responses(
        (status = 200, description = "Moderation logs retrieved successfully", body = ModerationLogListResponse),
        (status = 400, description = "Bad request - Invalid query parameters"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Moderation Logs"
)]
pub async fn get_moderation_logs(
    State(state): State<AppState>,
    RequiredSession(_session_context): RequiredSession,
    ValidatedQuery(payload): ValidatedQuery<GetModerationLogsRequest>,
) -> Result<ModerationLogListResponse, Errors> {
    service_get_moderation_logs(&state.read_db, payload).await
}
