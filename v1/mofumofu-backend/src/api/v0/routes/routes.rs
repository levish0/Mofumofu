use super::openapi::ApiDoc;
use crate::api::v0::routes::admin::routes::admin_routes;
use crate::api::v0::routes::auth::routes::auth_routes;
use crate::api::v0::routes::comment::routes::comment_routes;
use crate::api::v0::routes::draft::routes::draft_routes;
use crate::api::v0::routes::follow::routes::follow_routes;
use crate::api::v0::routes::hashtag::routes::hashtag_routes;
use crate::api::v0::routes::like::routes::like_routes;
use crate::api::v0::routes::post::routes::post_routes;
use crate::api::v0::routes::report::routes::report_routes;
use crate::api::v0::routes::user::routes::user_routes;
use crate::service::error::errors::handler_404;
use crate::state::AppState;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// API + Swagger UI 라우터 통합
pub fn api_routes() -> Router<AppState> {
    Router::new()
        .merge(SwaggerUi::new("/docs").url("/swagger.json", ApiDoc::openapi()))
        .nest("/v0", auth_routes())
        .nest("/v0", user_routes())
        .nest("/v0", post_routes())
        .nest("/v0", draft_routes())
        .nest("/v0", comment_routes())
        .nest("/v0", follow_routes())
        .nest("/v0", like_routes())
        .nest("/v0/hashtag", hashtag_routes())
        .nest("/v0", report_routes())
        .nest("/v0/admin", admin_routes())
        .fallback(handler_404)
}
