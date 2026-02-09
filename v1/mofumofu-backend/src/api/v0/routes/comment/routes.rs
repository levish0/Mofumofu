use crate::api::v0::routes::comment::create_comment::create_comment;
use crate::api::v0::routes::comment::delete_comment::delete_comment;
use crate::api::v0::routes::comment::get_comment_by_id::get_comment_by_id;
use crate::api::v0::routes::comment::get_comments::get_comments;
use crate::api::v0::routes::comment::get_replies::get_replies;
use crate::api::v0::routes::comment::update_comment::update_comment;
use crate::middleware::auth::access_jwt_auth;
use crate::state::AppState;
use axum::{
    Router,
    middleware::from_fn,
    routing::{delete as axum_delete, post, put},
};

pub fn comment_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/comment",
            post(create_comment).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/comment",
            put(update_comment).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/comment",
            axum_delete(delete_comment).route_layer(from_fn(access_jwt_auth)),
        )
        .route("/comment/get", post(get_comment_by_id))
        .route("/comment/list", post(get_comments))
        .route("/comment/replies", post(get_replies))
}
