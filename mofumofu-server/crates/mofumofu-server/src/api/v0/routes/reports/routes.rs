use super::create_report::create_report;
use super::get_reports::get_reports;
use super::update_report::update_report;
use crate::state::AppState;
use axum::{
    Router,
    routing::{patch, post},
};

pub fn reports_routes() -> Router<AppState> {
    Router::new()
        .route("/reports", post(create_report).get(get_reports))
        .route("/reports/{report_id}", patch(update_report))
}
