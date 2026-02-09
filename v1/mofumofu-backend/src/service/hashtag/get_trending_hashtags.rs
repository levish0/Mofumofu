use crate::dto::hashtag::response::trending_hashtags::TrendingHashtagsResponse;
use crate::repository::hashtag::get_popular_hashtags::repository_get_trending_hashtags;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::DatabaseConnection;

pub async fn service_get_trending_hashtags(
    conn: &DatabaseConnection,
    days: Option<i64>,
    limit: Option<u64>,
) -> ServiceResult<TrendingHashtagsResponse> {
    let days = days.unwrap_or(7); // 기본값: 최근 7일
    let limit = limit.unwrap_or(16); // 기본값: 상위 16개

    let hashtags = repository_get_trending_hashtags(conn, days, limit).await?;

    let hashtag_names = hashtags.into_iter().map(|h| h.name).collect();

    Ok(TrendingHashtagsResponse {
        hashtags: hashtag_names,
    })
}
