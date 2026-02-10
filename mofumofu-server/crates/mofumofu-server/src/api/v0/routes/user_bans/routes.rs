use super::ban_user::ban_user;
use super::get_user_bans::get_user_bans;
use super::unban_user::unban_user;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn user_bans_routes() -> Router<AppState> {
    Router::new()
        .route("/users/ban", post(ban_user))
        .route("/users/unban", post(unban_user))
        .route("/users/{user_id}/bans", get(get_user_bans))
}
