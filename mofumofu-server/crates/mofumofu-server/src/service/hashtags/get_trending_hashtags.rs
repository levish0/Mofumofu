use crate::repository::hashtags::repository_find_trending_hashtags;
use mofumofu_dto::hashtags::{HashtagResponse, TrendingHashtagsResponse};
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;

pub async fn service_get_trending_hashtags(
    conn: &DatabaseConnection,
    limit: u64,
) -> ServiceResult<TrendingHashtagsResponse> {
    let hashtags = repository_find_trending_hashtags(conn, limit).await?;

    let data: Vec<HashtagResponse> = hashtags.into_iter().map(HashtagResponse::from).collect();

    Ok(TrendingHashtagsResponse { data })
}
