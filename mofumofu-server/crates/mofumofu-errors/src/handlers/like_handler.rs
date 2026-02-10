use crate::errors::Errors;
use crate::protocol::like::*;
use axum::http::StatusCode;
use tracing::debug;

pub fn log_error(error: &Errors) {
    match error {
        Errors::LikeAlreadyLiked | Errors::LikeNotLiked | Errors::LikeTargetNotFound => {
            debug!("Client error: {:?}", error);
        }
        _ => {}
    }
}

pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::LikeAlreadyLiked => Some((StatusCode::CONFLICT, LIKE_ALREADY_LIKED, None)),
        Errors::LikeNotLiked => Some((StatusCode::NOT_FOUND, LIKE_NOT_LIKED, None)),
        Errors::LikeTargetNotFound => Some((StatusCode::NOT_FOUND, LIKE_TARGET_NOT_FOUND, None)),
        _ => None,
    }
}
