use super::create_post::create_post;
use super::delete_post::delete_post;
use super::get_post::get_post;
use super::get_post_by_slug::get_post_by_slug;
use super::get_post_feed::get_post_feed;
use super::get_posts::get_posts;
use super::increment_view::increment_view;
use super::update_post::update_post;
use super::upload_post_image::upload_post_image;
use crate::state::AppState;
use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use mofumofu_constants::POST_IMAGE_MAX_SIZE;

pub fn posts_routes() -> Router<AppState> {
    let image_upload_route = Router::new()
        .route("/posts/images", post(upload_post_image))
        .layer(DefaultBodyLimit::max(POST_IMAGE_MAX_SIZE));

    let crud_routes = Router::new()
        .route("/posts", post(create_post).get(get_posts))
        .route("/posts/feed", get(get_post_feed))
        .route("/posts/by-slug", get(get_post_by_slug))
        .route(
            "/posts/{post_id}",
            get(get_post).patch(update_post).delete(delete_post),
        )
        .route("/posts/{post_id}/view", post(increment_view));

    image_upload_route.merge(crud_routes)
}
