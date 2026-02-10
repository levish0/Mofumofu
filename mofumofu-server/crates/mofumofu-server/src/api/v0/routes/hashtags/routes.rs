use super::get_trending::get_trending;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn hashtags_routes() -> Router<AppState> {
    Router::new().route("/hashtags/trending", get(get_trending))
}
