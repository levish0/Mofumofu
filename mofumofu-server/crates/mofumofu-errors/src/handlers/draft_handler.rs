use crate::errors::Errors;
use crate::protocol::draft::*;
use axum::http::StatusCode;
use tracing::{debug, warn};

pub fn log_error(error: &Errors) {
    match error {
        Errors::DraftNotFound => {
            warn!("Resource not found: {:?}", error);
        }
        Errors::DraftMissingTitle | Errors::DraftMissingContent => {
            debug!("Client error: {:?}", error);
        }
        _ => {}
    }
}

pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::DraftNotFound => Some((StatusCode::NOT_FOUND, DRAFT_NOT_FOUND, None)),
        Errors::DraftMissingTitle => Some((StatusCode::BAD_REQUEST, DRAFT_MISSING_TITLE, None)),
        Errors::DraftMissingContent => Some((StatusCode::BAD_REQUEST, DRAFT_MISSING_CONTENT, None)),
        _ => None,
    }
}
