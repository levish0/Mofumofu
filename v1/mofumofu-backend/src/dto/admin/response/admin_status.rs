use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AdminStatusResponse {
    pub is_admin: bool,
}

impl IntoResponse for AdminStatusResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
