use crate::dto::auth::request::oauth::GithubLoginRequest;
use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::service::error::errors::Errors;
use crate::service::oauth::github_sign_in::service_github_sign_in;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use crate::utils::extract_ip_address::extract_ip_address;
use crate::utils::extract_user_agent::extract_user_agent;
use axum::extract::{ConnectInfo, State};
use axum::http::HeaderMap;
use axum_extra::TypedHeader;
use axum_extra::headers::UserAgent;
use std::net::SocketAddr;

// GitHub OAuth 로그인 엔드포인트
#[utoipa::path(
    post,
    path = "/v0/auth/github",
    request_body = GithubLoginRequest,
    responses(
        (status = 200, description = "GitHub OAuth login successful", body = AuthJWTResponse),
        (status = 400, description = "Invalid authorization code"),
        (status = 401, description = "OAuth authentication failed"),
        (status = 409, description = "Handle already exists"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn github_sign_in(
    user_agent: Option<TypedHeader<UserAgent>>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<GithubLoginRequest>,
) -> Result<AuthJWTResponse, Errors> {
    let ip_str = extract_ip_address(&headers, addr);
    let ua_str = extract_user_agent(user_agent);

    let res = service_github_sign_in(
        &state.conn,
        &state.cloudflare_r2,
        &state.http_client,
        Some(ua_str),
        Some(ip_str),
        &payload.code,
        payload.handle.as_deref(),
    )
    .await?;

    Ok(AuthJWTResponse {
        access_token: res.access_token,
        cookie_refresh_token: res.cookie_refresh_token,
    })
}
