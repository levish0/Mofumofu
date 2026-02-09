use crate::errors::Errors;
use crate::protocol::post::*;
use axum::http::StatusCode;
use tracing::warn;

pub fn log_error(error: &Errors) {
    match error {
        Errors::PostNotFound => {
            warn!("Resource not found: {:?}", error);
        }
        _ => {}
    }
}

pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::PostNotFound => Some((StatusCode::NOT_FOUND, POST_NOT_FOUND, None)),
        _ => None,
    }
}
