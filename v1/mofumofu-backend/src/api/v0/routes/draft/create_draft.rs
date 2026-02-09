use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::draft::request::create_draft::CreateDraftRequest;
use crate::dto::draft::response::create_draft::CreateDraftResponse;
use crate::service::auth::require_verified_user;
use crate::service::draft::create_draft::service_create_draft;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/draft",
    request_body = CreateDraftRequest,
    responses(
        (status = 201, description = "Draft created successfully", body = CreateDraftResponse),
        (status = StatusCode::BAD_REQUEST, description = "Draft limit exceeded: draft:limit_exceeded"),
        (status = StatusCode::CONFLICT, description = "Draft slug already exists: draft:slug_already_exists"),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized or email not verified"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Draft"
)]
pub async fn create_draft(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<CreateDraftRequest>,
) -> Result<CreateDraftResponse, Errors> {
    info!("Received POST request to create draft: {:?}", payload);
    let user_uuid = claims.sub.clone();

    require_verified_user(&state.conn, &claims).await?;

    let response = service_create_draft(&state.conn, payload, &user_uuid).await?;

    Ok(response)
}
