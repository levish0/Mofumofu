//! Redis cache key prefixes and helpers
//! Centralized constants for cache key naming to ensure consistency across codebase

use uuid::Uuid;

/// OAuth state TTL in seconds (5 minutes)
pub const OAUTH_STATE_TTL_SECONDS: u64 = 300;

/// OAuth state key prefix (stores PKCE verifier)
/// Format: "oauth:state:{uuid}"
pub const OAUTH_STATE_PREFIX: &str = "oauth:state:";

/// OAuth pending signup key prefix
/// Format: "oauth:pending:{uuid}"
pub const OAUTH_PENDING_PREFIX: &str = "oauth:pending:";

/// OAuth state key 생성
pub fn oauth_state_key(state: &str) -> String {
    format!("{}{}", OAUTH_STATE_PREFIX, state)
}

/// OAuth pending signup key 생성
pub fn oauth_pending_key(token: &str) -> String {
    format!("{}{}", OAUTH_PENDING_PREFIX, token)
}

/// Rendered post cache TTL (30 days)
pub const POST_RENDER_CACHE_TTL_SECONDS: u64 = 30 * 24 * 3600;

/// Rendered post cache key prefix
/// Format: "post:render:{post_id}"
pub const POST_RENDER_PREFIX: &str = "post:render:";

/// Generate post render cache key
pub fn post_render_key(post_id: Uuid) -> String {
    format!("{}{}", POST_RENDER_PREFIX, post_id)
}
