use crate::dto::user::request::get_profile::GetUserProfileRequest;
use crate::dto::user::response::handle_check::HandleCheckResponse;
use crate::service::error::errors::Errors;
use crate::service::user::service_check_handle_availability;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/user/check-handle",
    request_body = GetUserProfileRequest,
    responses(
        (status = StatusCode::OK, description = "Handle availability check result", body = HandleCheckResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "User"
)]
pub async fn check_handle_availability(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<GetUserProfileRequest>,
) -> Result<HandleCheckResponse, Errors> {
    info!(
        "Received POST request to check handle availability: {}",
        payload.handle
    );

    let is_available = service_check_handle_availability(&state.conn, &payload.handle).await?;
    Ok(HandleCheckResponse { is_available })
}
