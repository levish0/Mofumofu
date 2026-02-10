use super::check_follow_status::check_follow_status;
use super::create_follow::create_follow;
use super::delete_follow::delete_follow;
use super::get_followers::get_followers;
use super::get_following::get_following;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn follows_routes() -> Router<AppState> {
    Router::new()
        .route("/follows", post(create_follow).delete(delete_follow))
        .route("/follows/status", get(check_follow_status))
        .route("/follows/followers", get(get_followers))
        .route("/follows/following", get(get_following))
}
