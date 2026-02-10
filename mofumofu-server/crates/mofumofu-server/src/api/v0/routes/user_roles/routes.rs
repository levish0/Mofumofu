use super::get_user_roles::get_user_roles;
use super::grant_role::grant_role;
use super::revoke_role::revoke_role;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn user_roles_routes() -> Router<AppState> {
    Router::new()
        .route("/users/roles/grant", post(grant_role))
        .route("/users/roles/revoke", post(revoke_role))
        .route("/users/{user_id}/roles", get(get_user_roles))
}
