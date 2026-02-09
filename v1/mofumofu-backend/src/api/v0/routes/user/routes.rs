use crate::api::v0::routes::user::check_handle::check_handle_availability;
use crate::api::v0::routes::user::get_my_profile::get_my_profile;
use crate::api::v0::routes::user::get_profile::get_profile;
use crate::api::v0::routes::user::update_profile::update_profile;
use crate::api::v0::routes::user::upload_avatar::upload_avatar;
use crate::api::v0::routes::user::upload_banner::upload_banner;
use crate::middleware::auth::access_jwt_auth;
use crate::state::AppState;
use axum::Router;
use axum::routing::{get, post, put};

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/user/check-handle", post(check_handle_availability))
        .route("/user/profile", post(get_profile))
        // 보호된 사용자 프로필 API
        .route(
            "/user/my_profile",
            get(get_my_profile).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/user/profile",
            put(update_profile).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        // 이미지 업로드 API
        .route(
            "/user/profile/avatar",
            post(upload_avatar).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
        .route(
            "/user/profile/banner",
            post(upload_banner).route_layer(axum::middleware::from_fn(access_jwt_auth)),
        )
}
