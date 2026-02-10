use crate::errors::Errors;
use crate::protocol::report::*;
use axum::http::StatusCode;
use tracing::{debug, warn};

pub fn log_error(error: &Errors) {
    match error {
        Errors::ReportNotFound => {
            warn!("Resource not found: {:?}", error);
        }
        Errors::ReportAlreadyExists => {
            debug!("Client error: {:?}", error);
        }
        _ => {}
    }
}

pub fn map_response(error: &Errors) -> Option<(StatusCode, &'static str, Option<String>)> {
    match error {
        Errors::ReportNotFound => Some((StatusCode::NOT_FOUND, REPORT_NOT_FOUND, None)),
        Errors::ReportAlreadyExists => Some((StatusCode::CONFLICT, REPORT_ALREADY_EXISTS, None)),
        _ => None,
    }
}
