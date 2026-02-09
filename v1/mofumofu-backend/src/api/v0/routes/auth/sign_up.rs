use crate::dto::user::request::create::CreateUserRequest;
use crate::service::auth::service_sign_up;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/auth/sign_up",
    request_body = CreateUserRequest,
    responses(
        (status = 204, description = "User created successfully, verification email sent"),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "Handle or email already exists"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn sign_up(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received POST request to sign up: {:?}", payload);

    service_sign_up(&state, payload).await?;

    Ok(StatusCode::NO_CONTENT)
}
