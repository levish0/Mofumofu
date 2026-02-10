use super::create_draft::create_draft;
use super::delete_draft::delete_draft;
use super::get_draft::get_draft;
use super::get_drafts::get_drafts;
use super::publish_draft::publish_draft;
use super::update_draft::update_draft;
use crate::state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn drafts_routes() -> Router<AppState> {
    Router::new()
        .route("/drafts", post(create_draft).get(get_drafts))
        .route(
            "/drafts/{draft_id}",
            get(get_draft).patch(update_draft).delete(delete_draft),
        )
        .route("/drafts/{draft_id}/publish", post(publish_draft))
}
