use crate::extractors::RequiredSession;
use crate::service::user_roles::service_grant_role;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mofumofu_dto::user_roles::GrantRoleRequest;
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/users/roles/grant",
    request_body = GrantRoleRequest,
    responses(
        (status = 201, description = "Role granted successfully", body = mofumofu_dto::user_roles::UserRoleResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 403, description = "Forbidden - Insufficient privileges"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Roles"
)]
pub async fn grant_role(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<GrantRoleRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_grant_role(&state.write_db, session_context.user_id, payload).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
