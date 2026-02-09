use crate::entity::common::OAuthProvider;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct OAuthConnectionsResponse {
    pub connections: Vec<OAuthProvider>,
    pub is_oauth_only: bool,
}

impl IntoResponse for OAuthConnectionsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
