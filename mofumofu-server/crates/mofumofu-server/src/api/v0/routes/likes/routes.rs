use super::check_like_status::check_like_status;
use super::create_like::create_like;
use super::delete_like::delete_like;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn likes_routes() -> Router<AppState> {
    Router::new()
        .route("/likes", post(create_like).delete(delete_like))
        .route("/likes/status", get(check_like_status))
}
