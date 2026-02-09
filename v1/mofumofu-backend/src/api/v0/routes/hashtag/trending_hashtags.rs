use crate::dto::hashtag::request::trending_hashtags::TrendingHashtagsRequest;
use crate::dto::hashtag::response::trending_hashtags::TrendingHashtagsResponse;
use crate::service::error::errors::Errors;
use crate::service::hashtag::get_trending_hashtags::service_get_trending_hashtags;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;

#[utoipa::path(
    post,
    path = "/v0/hashtag/trending",
    request_body = TrendingHashtagsRequest,
    responses(
        (status = StatusCode::OK, description = "Trending hashtags retrieved successfully", body = TrendingHashtagsResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Hashtag"
)]
pub async fn trending_hashtags(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<TrendingHashtagsRequest>,
) -> Result<TrendingHashtagsResponse, Errors> {
    let response = service_get_trending_hashtags(&state.conn, payload.days, payload.limit).await?;

    Ok(response)
}
