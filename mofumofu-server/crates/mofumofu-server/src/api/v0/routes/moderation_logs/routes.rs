use super::get_moderation_logs::get_moderation_logs;
use crate::state::AppState;
use axum::{Router, routing::get};

pub fn moderation_logs_routes() -> Router<AppState> {
    Router::new().route("/moderation-logs", get(get_moderation_logs))
}
