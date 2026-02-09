use crate::api::v0::routes::report::create_report::create_report;
use crate::api::v0::routes::report::get_reports::get_reports;
use crate::api::v0::routes::report::process_report::process_report;
use crate::middleware::auth::{access_jwt_auth, optional_access_jwt_auth};
use crate::state::AppState;
use axum::{
    Router,
    middleware::from_fn,
    routing::{post, put},
};

pub fn report_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/report",
            post(create_report).route_layer(from_fn(optional_access_jwt_auth)), // 선택적 인증
        )
        .route(
            "/report/list",
            post(get_reports).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/report/process",
            put(process_report).route_layer(from_fn(access_jwt_auth)),
        )
}
