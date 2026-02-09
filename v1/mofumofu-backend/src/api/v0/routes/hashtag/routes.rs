use super::trending_hashtags::trending_hashtags;
use crate::state::AppState;
use axum::Router;
use axum::routing::post;

pub fn hashtag_routes() -> Router<AppState> {
    Router::new().route("/trending", post(trending_hashtags))
}
