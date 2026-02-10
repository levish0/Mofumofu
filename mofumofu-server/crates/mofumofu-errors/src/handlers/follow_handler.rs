use crate::errors::Errors;
use crate::protocol::follow::*;
use axum::http::StatusCode;
use tracing::debug;

pub fn log_error(error: &Errors) {
    match error {
        Errors::FollowSelfFollow | Errors::FollowAlreadyFollowing | Errors::FollowNotFollowing => {
            debug!("Client error: {:?}", error);
        }
        _ => {}
    }
}

pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::FollowSelfFollow => Some((StatusCode::BAD_REQUEST, FOLLOW_SELF_FOLLOW, None)),
        Errors::FollowAlreadyFollowing => {
            Some((StatusCode::CONFLICT, FOLLOW_ALREADY_FOLLOWING, None))
        }
        Errors::FollowNotFollowing => Some((StatusCode::NOT_FOUND, FOLLOW_NOT_FOLLOWING, None)),
        _ => None,
    }
}
