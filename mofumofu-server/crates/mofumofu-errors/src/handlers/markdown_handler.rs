use crate::errors::Errors;
use crate::protocol::markdown::*;
use axum::http::StatusCode;
use tracing::error;

pub fn log_error(err: &Errors) {
    match err {
        Errors::MarkdownRenderFailed => {
            error!("Markdown render service failed: {:?}", err);
        }
        _ => {}
    }
}

pub fn map_response(err: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match err {
        Errors::MarkdownRenderFailed => {
            Some((StatusCode::BAD_GATEWAY, MARKDOWN_RENDER_FAILED, None))
        }
        _ => None,
    }
}
