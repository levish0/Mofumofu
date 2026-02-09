use crate::errors::Errors;
use crate::protocol::comment::*;
use axum::http::StatusCode;
use tracing::warn;

pub fn log_error(error: &Errors) {
    match error {
        Errors::CommentNotFound => {
            warn!("Resource not found: {:?}", error);
        }
        _ => {}
    }
}

pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::CommentNotFound => Some((StatusCode::NOT_FOUND, COMMENT_NOT_FOUND, None)),
        _ => None,
    }
}
