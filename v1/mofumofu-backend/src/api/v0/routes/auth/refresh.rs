use crate::dto::auth::internal::refresh_token::RefreshTokenContext;
use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::service::auth::service_refresh;
use crate::service::error::errors::Errors;
use crate::state::AppState;
use crate::utils::extract_ip_address::extract_ip_address;
use crate::utils::extract_user_agent::extract_user_agent;
use axum::Extension;
use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use axum_extra::TypedHeader;
use axum_extra::headers::UserAgent;
use std::net::SocketAddr;

#[utoipa::path(
    post,
    path = "/v0/auth/refresh",
    responses(
        (status = StatusCode::OK, description = "Token refresh successful", body = AuthJWTResponse),
        (status = StatusCode::UNAUTHORIZED, description = "Refresh token cookie exists but is invalid or malformed"),
        (status = StatusCode::BAD_REQUEST, description = "No refresh token cookie found"),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error")
    ),
    security(
        ("refresh_token_cookie" = [])
    ),
    tag = "Auth"
)]
pub async fn refresh(
    user_agent: Option<TypedHeader<UserAgent>>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Extension(ctx): Extension<RefreshTokenContext>,
) -> Result<AuthJWTResponse, Errors> {
    let ip_str = extract_ip_address(&headers, addr);
    let ua_str = extract_user_agent(user_agent);

    let refresh_token = ctx.token;
    let refresh_token_claims = ctx.claims;
    let res = service_refresh(
        &state.conn,
        Some(ua_str),
        Some(ip_str),
        refresh_token,
        refresh_token_claims,
    )
    .await?;

    Ok(AuthJWTResponse {
        access_token: res.access_token,
        cookie_refresh_token: res.cookie_refresh_token,
    })
}
