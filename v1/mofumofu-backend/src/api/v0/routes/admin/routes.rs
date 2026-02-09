use axum::{
    Router,
    routing::{get, post},
};

use crate::{middleware::auth::access_jwt_auth, state::AppState};

use super::{
    check_admin_status::check_admin_status, cleanup_expired_tokens::cleanup_expired_tokens,
    cleanup_old_events::cleanup_old_events, meilisearch_health::meilisearch_health,
    reindex_all_posts::reindex_all_posts, search_stats::search_stats,
    sync_all_counts::sync_all_counts, sync_follows::sync_follows, sync_likes::sync_likes,
};

pub fn admin_routes() -> Router<AppState> {
    Router::new()
        // Admin status check
        .route("/status", get(check_admin_status))
        // Search management endpoints
        .route("/search/reindex-all", post(reindex_all_posts))
        .route("/search/health", post(meilisearch_health))
        .route("/search/stats", post(search_stats))
        // Count sync endpoints
        .route("/sync/likes", post(sync_likes))
        .route("/sync/follows", post(sync_follows))
        .route("/sync/all", post(sync_all_counts))
        // Cleanup endpoints
        .route("/cleanup/tokens", post(cleanup_expired_tokens))
        .route("/cleanup/events", post(cleanup_old_events))
        // All admin routes require authentication
        .layer(axum::middleware::from_fn(access_jwt_auth))
}
