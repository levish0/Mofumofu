use crate::api::v0::routes::auth::forgot_password::forgot_password;
use crate::api::v0::routes::auth::get_oauth_connections::get_oauth_connections;
use crate::api::v0::routes::auth::github::github_sign_in;
use crate::api::v0::routes::auth::google::google_sign_in;
use crate::api::v0::routes::auth::link_oauth::link_oauth;
use crate::api::v0::routes::auth::refresh::refresh;
use crate::api::v0::routes::auth::resend_verification::resend_verification;
use crate::api::v0::routes::auth::reset_password::reset_password;
use crate::api::v0::routes::auth::set_password::set_password;
use crate::api::v0::routes::auth::sign_in::sign_in;
use crate::api::v0::routes::auth::sign_out::sign_out;
use crate::api::v0::routes::auth::sign_up::sign_up;
use crate::api::v0::routes::auth::unlink_oauth::unlink_oauth;
use crate::api::v0::routes::auth::verify_email::verify_email;
use crate::middleware::auth::{access_jwt_auth, refresh_jwt_auth};
use crate::state::AppState;
use axum::Router;
use axum::routing::{delete, get, post};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/sign_in", post(sign_in))
        .route("/auth/sign_up", post(sign_up))
        .route("/auth/verify_email", post(verify_email))
        .route("/auth/resend_verification", post(resend_verification))
        .route("/auth/forgot_password", post(forgot_password))
        .route("/auth/reset_password", post(reset_password))
        .route(
            "/auth/set_password",
            post(set_password).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/auth/link_oauth",
            post(link_oauth).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/auth/oauth-connections",
            get(get_oauth_connections).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/auth/unlink-oauth",
            delete(unlink_oauth).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/auth/sign_out",
            post(sign_out).route_layer(axum::middleware::from_fn(refresh_jwt_auth)),
        )
        .route(
            "/auth/refresh",
            post(refresh).route_layer(axum::middleware::from_fn(refresh_jwt_auth)),
        )
        .route("/auth/google", post(google_sign_in))
        .route("/auth/github", post(github_sign_in))
}
