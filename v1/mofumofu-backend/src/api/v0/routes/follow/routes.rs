use crate::api::v0::routes::follow::check_follow_status::api_check_follow_status;
use crate::api::v0::routes::follow::create_follow::api_create_follow;
use crate::api::v0::routes::follow::delete_follow::api_delete_follow;
use crate::api::v0::routes::follow::get_follower_count::api_get_follower_count;
use crate::api::v0::routes::follow::get_followers_list::get_followers;
use crate::api::v0::routes::follow::get_following_count::api_get_following_count;
use crate::api::v0::routes::follow::get_following_list::get_following;
use crate::middleware::auth::access_jwt_auth;
use crate::state::AppState;
use axum::Router;
use axum::middleware::from_fn;
use axum::routing::{delete as axum_delete, get, post};

pub fn follow_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/follow",
            post(api_create_follow).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/follow",
            axum_delete(api_delete_follow).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/follow/status",
            post(api_check_follow_status).route_layer(from_fn(access_jwt_auth)),
        )
        .route("/follow/follower-count", post(api_get_follower_count))
        .route("/follow/following-count", post(api_get_following_count))
        .route("/followers/{handle}", get(get_followers))
        .route("/following/{handle}", get(get_following))
}
