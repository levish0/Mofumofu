use crate::api::v0::routes::post::create_post::create_post;
use crate::api::v0::routes::post::delete_post::delete_post;
use crate::api::v0::routes::post::get_post::get_post;
use crate::api::v0::routes::post::get_post_by_handle_and_slug::get_post_by_handle_and_slug;
use crate::api::v0::routes::post::get_post_for_edit::get_post_for_edit;
use crate::api::v0::routes::post::get_posts::get_posts;
use crate::api::v0::routes::post::get_user_posts::get_user_posts;
use crate::api::v0::routes::post::increment_view::increment_view;
use crate::api::v0::routes::post::search_posts::search_posts;
use crate::api::v0::routes::post::update_post::update_post;
use crate::api::v0::routes::post::upload_image::upload_image;
use crate::api::v0::routes::post::upload_thumbnail::upload_thumbnail;
use crate::middleware::anonymous_user::anonymous_user_middleware;
use crate::{middleware::auth::access_jwt_auth, state::AppState};
use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, post, put},
};

pub fn post_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/post",
            post(create_post).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/post",
            put(update_post).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/post",
            delete(delete_post).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/post/thumbnail",
            post(upload_thumbnail).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/post/image",
            post(upload_image).route_layer(from_fn(access_jwt_auth)),
        )
        .route("/post/get", post(get_post))
        .route(
            "/post/get_by_handle_and_slug",
            post(get_post_by_handle_and_slug),
        )
        .route(
            "/post/edit",
            post(get_post_for_edit).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/post/view",
            post(increment_view).route_layer(axum::middleware::from_fn(anonymous_user_middleware)),
        )
        .route("/posts", post(get_posts))
        .route("/posts/user", post(get_user_posts))
        .route("/posts/search", post(search_posts))
}
