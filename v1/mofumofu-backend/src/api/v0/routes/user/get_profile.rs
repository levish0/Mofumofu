use crate::dto::user::request::get_profile::GetUserProfileRequest;
use crate::dto::user::response::info::UserInfoResponse;
use crate::service::error::errors::Errors;
use crate::service::user::service_get_user_by_handle;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/user/profile",
    request_body = GetUserProfileRequest,
    responses(
        (status = StatusCode::OK, description = "Successfully retrieved user information", body = UserInfoResponse),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "User"
)]
pub async fn get_profile(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<GetUserProfileRequest>,
) -> Result<UserInfoResponse, Errors> {
    info!("Received GET request for user with ID: {}", payload.handle);

    let user = service_get_user_by_handle(&state.conn, &payload.handle).await?;
    Ok(user)
}
