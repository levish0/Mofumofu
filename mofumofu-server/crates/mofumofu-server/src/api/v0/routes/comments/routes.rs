use super::create_comment::create_comment;
use super::delete_comment::delete_comment;
use super::get_comments::get_comments;
use super::update_comment::update_comment;
use crate::state::AppState;
use axum::{
    Router,
    routing::{patch, post},
};

pub fn comments_routes() -> Router<AppState> {
    Router::new()
        .route("/comments", post(create_comment).get(get_comments))
        .route(
            "/comments/{comment_id}",
            patch(update_comment).delete(delete_comment),
        )
}
