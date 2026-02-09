use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::auth::response::oauth_connections::OAuthConnectionsResponse;
use crate::service::auth::service_get_oauth_connections;
use crate::service::error::errors::Errors;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use tracing::info;

#[utoipa::path(
    get,
    path = "/v0/auth/oauth-connections",
    responses(
        (status = 200, description = "OAuth connections retrieved successfully", body = OAuthConnectionsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_oauth_connections(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
) -> Result<OAuthConnectionsResponse, Errors> {
    info!(
        "Received GET request to retrieve OAuth connections for user: {}",
        claims.sub
    );

    service_get_oauth_connections(&state.conn, &claims.sub).await
}
