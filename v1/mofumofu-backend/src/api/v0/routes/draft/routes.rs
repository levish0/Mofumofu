use crate::api::v0::routes::draft::create_draft::create_draft;
use crate::api::v0::routes::draft::delete_draft::delete_draft;
use crate::api::v0::routes::draft::get_draft::get_draft;
use crate::api::v0::routes::draft::get_drafts::get_drafts;
use crate::api::v0::routes::draft::update_draft::update_draft;
use crate::{middleware::auth::access_jwt_auth, state::AppState};
use axum::{Router, middleware::from_fn, routing::post};

pub fn draft_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/draft",
            post(create_draft).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/draft/get",
            post(get_draft).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/draft/update",
            post(update_draft).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/draft/delete",
            post(delete_draft).route_layer(from_fn(access_jwt_auth)),
        )
        .route(
            "/drafts",
            post(get_drafts).route_layer(from_fn(access_jwt_auth)),
        )
}
