use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::post::request::create_post::CreatePostRequest;
use crate::dto::post::response::create_post::CreatePostResponse;
use crate::middleware::auth::access_jwt_auth;
use crate::service::auth::require_verified_user;
use crate::service::error::errors::Errors;
use crate::service::post::create_post::service_create_post;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware::from_fn;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Extension, Router};
use tracing::info;

pub fn post_routes() -> Router<AppState> {
    Router::new().route(
        "/post",
        post(create_post).route_layer(from_fn(access_jwt_auth)),
    )
}

#[utoipa::path(
    post,
    path = "/v0/post",
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Post created successfully", body = CreatePostResponse),
        (status = StatusCode::BAD_REQUEST, description = "Markdown render failed: markdown:render_failed"),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized or email not verified"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Post"
)]
pub async fn create_post(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<CreatePostRequest>,
) -> Result<CreatePostResponse, Errors> {
    info!("Received POST request to create post: {:?}", payload);
    let user_uuid = claims.sub.clone();

    require_verified_user(&state.conn, &claims).await?;

    let response =
        service_create_post(&state.conn, &state.http_client, payload, &user_uuid).await?;

    Ok(response)
}
