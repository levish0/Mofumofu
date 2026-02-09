use crate::config::db_config::DbConfig;
use crate::dto::auth::internal::anonymous_user::AnonymousUserContext;
use axum::body::Body;
use axum::http::HeaderValue;
use axum::http::Request;
use axum::http::header::{COOKIE, SET_COOKIE};
use axum::middleware::Next;
use axum::response::Response;
use cookie::time::Duration;
use cookie::{Cookie, SameSite};
use uuid::Uuid;

pub const ANONYMOUS_USER_COOKIE_NAME: &str = "anonymous_user_id";

pub async fn anonymous_user_middleware(mut req: Request<Body>, next: Next) -> Response {
    // 쿠키에서 anonymous_user_id 확인
    let cookie_header = req.headers().get(COOKIE);
    let mut anonymous_user_id = None;
    let mut has_anonymous_id = false;

    if let Some(cookie_value) = cookie_header {
        if let Ok(cookie_str) = cookie_value.to_str() {
            // 쿠키 파싱해서 anonymous_user_id 찾기
            for cookie_pair in cookie_str.split(';') {
                let cookie_pair = cookie_pair.trim();
                if let Some((key, value)) = cookie_pair.split_once('=') {
                    if key.trim() == ANONYMOUS_USER_COOKIE_NAME {
                        anonymous_user_id = Some(value.trim().to_string());
                        has_anonymous_id = true;
                        break;
                    }
                }
            }
        }
    }

    // 익명 사용자 ID가 없으면 생성
    let final_anonymous_id = if let Some(id) = anonymous_user_id {
        id
    } else {
        Uuid::new_v4().to_string()
    };

    // Extension에 익명 사용자 컨텍스트 추가
    req.extensions_mut().insert(AnonymousUserContext {
        anonymous_user_id: final_anonymous_id.clone(),
    });

    let mut response = next.run(req).await;

    // 쿠키가 없었다면 새로 생성해서 설정
    if !has_anonymous_id {
        let is_dev = DbConfig::get().is_dev;

        let same_site_attribute = if is_dev {
            SameSite::None
        } else {
            SameSite::Lax
        };

        let cookie = Cookie::build((ANONYMOUS_USER_COOKIE_NAME, final_anonymous_id))
            .http_only(true)
            .secure(true)
            .same_site(same_site_attribute)
            .path("/")
            .max_age(Duration::days(365)) // 1년
            .build();

        response.headers_mut().insert(
            SET_COOKIE,
            HeaderValue::from_str(&cookie.to_string()).unwrap(),
        );
    }

    response
}
