use crate::extractors::RequiredSession;
use crate::service::user_roles::service_revoke_role;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use mofumofu_dto::user_roles::RevokeRoleRequest;
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/users/roles/revoke",
    request_body = RevokeRoleRequest,
    responses(
        (status = 204, description = "Role revoked successfully"),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 403, description = "Forbidden - Insufficient privileges"),
        (status = 404, description = "Not Found - User does not have this role"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "User Roles"
)]
pub async fn revoke_role(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<RevokeRoleRequest>,
) -> Result<StatusCode, Errors> {
    service_revoke_role(
        &state.write_db,
        session_context.user_id,
        payload.user_id,
        payload.role,
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}
