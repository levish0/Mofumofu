use crate::errors::Errors;
use crate::protocol::draft::*;
use axum::http::StatusCode;
use tracing::warn;

pub fn log_error(error: &Errors) {
    match error {
        Errors::DraftNotFound => {
            warn!("Resource not found: {:?}", error);
        }
        _ => {}
    }
}

pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::DraftNotFound => Some((StatusCode::NOT_FOUND, DRAFT_NOT_FOUND, None)),
        _ => None,
    }
}
