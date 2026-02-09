use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::post::request::delete_post::DeletePostRequest;
use crate::service::auth::require_verified_user;
use crate::service::error::errors::Errors;
use crate::service::post::delete_post::service_delete_post;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    delete,
    path = "/v0/post",
    request_body = DeletePostRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "Post deleted successfully"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized or email not verified"),
        (status = StatusCode::NOT_FOUND, description = "Post not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Post"
)]
#[axum::debug_handler]
pub async fn delete_post(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<DeletePostRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received DELETE request to delete post: {:?}", payload);
    let user_uuid = claims.sub.clone();

    require_verified_user(&state.conn, &claims).await?;

    service_delete_post(&state.conn, &state.http_client, payload, &user_uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}
