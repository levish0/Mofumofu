use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::like::request::create_like::CreateLikeRequest;
use crate::service::error::errors::Errors;
use crate::service::like::create_post_like::service_create_post_like;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/like",
    request_body = CreateLikeRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "Post liked successfully"),
        (status = StatusCode::NOT_FOUND, description = "Post not found"),
        (status = StatusCode::CONFLICT, description = "Already liked: like:already_exists"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Like"
)]
pub async fn create_like(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<CreateLikeRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received request to like post: {:?}", payload);
    let user_uuid = claims.sub.clone();

    service_create_post_like(&state.conn, &user_uuid, &payload.post_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
