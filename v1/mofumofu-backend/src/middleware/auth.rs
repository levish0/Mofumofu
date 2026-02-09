use crate::dto::auth::internal::refresh_token::RefreshTokenContext;
use crate::dto::auth::response::sign_out::SignOutResponse;
use crate::service::auth::jwt::{decode_access_token, decode_refresh_token};
use crate::service::error::errors::Errors;
use axum::body::Body;
use axum::http::Request;
use axum::http::header::COOKIE;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

pub async fn access_jwt_auth(mut req: Request<Body>, next: Next) -> Result<Response, Errors> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let token = if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            header.trim_start_matches("Bearer ").to_string()
        } else {
            return Err(Errors::UserUnauthorized);
        }
    } else {
        return Err(Errors::UserUnauthorized);
    };

    let token_data = match decode_access_token(&token) {
        Ok(data) => data,
        Err(_) => return Err(Errors::UserUnauthorized),
    };
    req.extensions_mut().insert(token_data.claims);
    Ok(next.run(req).await)
}

// 선택적 인증 미들웨어: 토큰이 있으면 검증해서 Extension에 추가, 없으면 그냥 진행
pub async fn optional_access_jwt_auth(mut req: Request<Body>, next: Next) -> Response {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            let token = header.trim_start_matches("Bearer ").to_string();

            // 토큰이 유효하면 Extension에 추가
            if let Ok(token_data) = decode_access_token(&token) {
                req.extensions_mut().insert(token_data.claims);
            }
            // 토큰이 유효하지 않아도 에러를 발생시키지 않고 진행
        }
    }
    // 토큰이 없어도 에러를 발생시키지 않고 진행

    next.run(req).await
}

pub async fn refresh_jwt_auth(mut req: Request<Body>, next: Next) -> Response {
    // 쿠키에서 refresh_token 추출
    let cookie_header = req.headers().get(COOKIE);

    let refresh_token = if let Some(cookie_value) = cookie_header {
        let cookie_str = match cookie_value.to_str() {
            Ok(s) => s,
            Err(_) => return SignOutResponse.into_response(),
        };

        // 쿠키 파싱해서 refresh_token 찾기
        let mut token = None;
        for cookie_pair in cookie_str.split(';') {
            let cookie_pair = cookie_pair.trim();
            if let Some((key, value)) = cookie_pair.split_once('=') {
                if key.trim() == "refresh_token" {
                    token = Some(value.trim());
                    break;
                }
            }
        }

        match token {
            Some(t) => t,
            None => return SignOutResponse.into_response(),
        }
    } else {
        return SignOutResponse.into_response();
    };

    let token_data = match decode_refresh_token(refresh_token) {
        Ok(data) => data,
        Err(_) => return SignOutResponse.into_response(),
    };

    let context = RefreshTokenContext {
        token: refresh_token.to_string(),
        claims: token_data.claims,
    };
    req.extensions_mut().insert(context);
    next.run(req).await
}
