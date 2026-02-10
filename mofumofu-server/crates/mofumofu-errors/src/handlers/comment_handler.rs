use crate::errors::Errors;
use crate::protocol::comment::*;
use axum::http::StatusCode;
use tracing::{debug, warn};

pub fn log_error(error: &Errors) {
    match error {
        Errors::CommentNotFound | Errors::CommentParentNotFound => {
            warn!("Resource not found: {:?}", error);
        }
        Errors::CommentDepthExceeded | Errors::CommentPostMismatch => {
            debug!("Client error: {:?}", error);
        }
        _ => {}
    }
}

pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::CommentNotFound => Some((StatusCode::NOT_FOUND, COMMENT_NOT_FOUND, None)),
        Errors::CommentDepthExceeded => {
            Some((StatusCode::BAD_REQUEST, COMMENT_DEPTH_EXCEEDED, None))
        }
        Errors::CommentParentNotFound => {
            Some((StatusCode::NOT_FOUND, COMMENT_PARENT_NOT_FOUND, None))
        }
        Errors::CommentPostMismatch => Some((StatusCode::BAD_REQUEST, COMMENT_POST_MISMATCH, None)),
        _ => None,
    }
}
