use super::PostSortOrder;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SearchPostsRequest {
    /// 검색 쿼리 (제목, 내용, 해시태그, 사용자명에서 검색)
    pub query: Option<String>,

    /// 특정 해시태그로 필터링
    #[validate(length(max = 8, message = "Maximum 8 hashtags allowed for filtering."))]
    pub hashtags: Option<Vec<String>>,

    /// 특정 사용자로 필터링
    pub user_handle: Option<String>,

    /// 날짜 범위 필터링 - 시작일
    pub date_from: Option<DateTime<Utc>>,

    /// 날짜 범위 필터링 - 종료일  
    pub date_to: Option<DateTime<Utc>>,

    /// 최소 좋아요 수
    #[validate(range(min = 0, message = "Min likes must be non-negative."))]
    pub min_likes: Option<i32>,

    /// 정렬 방식
    pub sort: Option<PostSortOrder>,

    /// 페이지 번호
    #[validate(range(min = 1, message = "Page must be greater than 0."))]
    pub page: Option<u32>,

    /// 페이지 크기
    #[validate(range(min = 1, max = 20, message = "Page size must be between 1 and 20."))]
    pub page_size: Option<u32>,
}

impl Default for SearchPostsRequest {
    fn default() -> Self {
        Self {
            query: None,
            hashtags: None,
            user_handle: None,
            date_from: None,
            date_to: None,
            min_likes: None,
            sort: Some(PostSortOrder::Latest),
            page: Some(1),
            page_size: Some(20),
        }
    }
}
