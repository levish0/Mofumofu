use crate::config::db_config::DbConfig;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header::SET_COOKIE;
use axum::response::{IntoResponse, Response};
use cookie::time::Duration;
use cookie::{Cookie, SameSite};

pub struct SignOutResponse;

impl IntoResponse for SignOutResponse {
    fn into_response(self) -> Response {
        let is_dev = DbConfig::get().is_dev;

        let same_site_attribute = if is_dev {
            SameSite::None
        } else {
            SameSite::Lax
        };

        // 만료된 refresh token 쿠키 설정 (max_age를 0으로 설정하여 즉시 만료)
        let expired_cookie = Cookie::build(("refresh_token", ""))
            .http_only(true)
            .secure(true)
            .same_site(same_site_attribute)
            .path("/")
            .max_age(Duration::ZERO)
            .build();

        let mut response = StatusCode::NO_CONTENT.into_response();

        response.headers_mut().insert(
            SET_COOKIE,
            HeaderValue::from_str(&expired_cookie.to_string()).unwrap(),
        );

        response
    }
}
