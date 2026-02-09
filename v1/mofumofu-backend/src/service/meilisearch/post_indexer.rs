use crate::connection::meilisearch::{MeilisearchClient, MeilisearchPost, MeilisearchPostId};
use tracing::warn;

pub async fn setup_posts_index(
    meilisearch: &MeilisearchClient,
) -> Result<(), meilisearch_sdk::errors::Error> {
    let posts_index = meilisearch.get_client().index("posts");

    // 검색 가능한 필드 설정
    posts_index
        .set_searchable_attributes(&[
            "title",
            "content",
            "summary",
            "hashtags",
            "user_handle",
            "user_name",
        ])
        .await?;

    // 필터링 가능한 필드 설정
    posts_index
        .set_filterable_attributes(&[
            "user_id",
            "user_handle",
            "hashtags",
            "created_at",
            "like_count",
            "view_count",
        ])
        .await?;

    // 정렬 가능한 필드 설정
    posts_index
        .set_sortable_attributes(&["created_at", "like_count", "view_count", "comment_count"])
        .await?;

    Ok(())
}

pub async fn index_post(meilisearch: &MeilisearchClient, post: &MeilisearchPost) {
    let posts_index = meilisearch.get_client().index("posts");
    if let Err(e) = posts_index.add_documents(&[post], Some("id")).await {
        warn!("Failed to index post in Meilisearch: {}", e);
    }
}

pub async fn update_post(meilisearch: &MeilisearchClient, post: &MeilisearchPost) {
    let posts_index = meilisearch.get_client().index("posts");
    if let Err(e) = posts_index.add_or_replace(&[post], Some("id")).await {
        warn!("Failed to update post in Meilisearch: {}", e);
    }
}

pub async fn delete_post(meilisearch: &MeilisearchClient, post_id: &str) {
    let posts_index = meilisearch.get_client().index("posts");
    if let Err(e) = posts_index.delete_document(post_id).await {
        warn!("Failed to delete post from Meilisearch: {}", e);
    }
}

pub async fn search_posts(
    meilisearch: &MeilisearchClient,
    query: Option<&str>,
    hashtags: Option<&[String]>,
    user_handle: Option<&str>,
    date_from: Option<i64>,
    date_to: Option<i64>,
    min_likes: Option<i32>,
    sort: &str,
    page: u32,
    page_size: u32,
) -> Result<(Vec<String>, u64), meilisearch_sdk::errors::Error> {
    let posts_index = meilisearch.get_client().index("posts");
    let offset = (page - 1) * page_size;

    // 필터 조건 구성
    let mut filters = Vec::new();

    if let Some(tags) = hashtags {
        if !tags.is_empty() {
            let hashtag_filters: Vec<String> = tags
                .iter()
                .map(|tag| format!("hashtags = '{}'", tag))
                .collect();
            filters.push(format!("({})", hashtag_filters.join(" OR ")));
        }
    }

    if let Some(handle) = user_handle {
        filters.push(format!("user_handle = '{}'", handle));
    }

    if let Some(from_timestamp) = date_from {
        filters.push(format!("created_at >= {}", from_timestamp));
    }

    if let Some(to_timestamp) = date_to {
        filters.push(format!("created_at <= {}", to_timestamp));
    }

    if let Some(min_likes_count) = min_likes {
        filters.push(format!("like_count >= {}", min_likes_count));
    }

    // 검색 쿼리 빌드
    let query_str = query.unwrap_or("");
    let filter_str = if filters.is_empty() {
        None
    } else {
        Some(filters.join(" AND "))
    };

    // 정렬 설정
    let sort_criteria: Vec<&str> = match sort {
        "popular" => vec!["like_count:desc", "created_at:desc"],
        "oldest" => vec!["created_at:asc"],
        _ => vec!["created_at:desc"], // default: latest
    };

    // 검색 쿼리 구성 및 실행 (ID만 반환)
    let search_results = if let Some(filter) = filter_str {
        posts_index
            .search()
            .with_query(query_str)
            .with_filter(&filter)
            .with_sort(&sort_criteria)
            .with_offset(offset as usize)
            .with_limit(page_size as usize)
            .with_attributes_to_retrieve(meilisearch_sdk::search::Selectors::Some(&["id"]))
            .execute::<MeilisearchPostId>()
            .await?
    } else {
        posts_index
            .search()
            .with_query(query_str)
            .with_sort(&sort_criteria)
            .with_offset(offset as usize)
            .with_limit(page_size as usize)
            .with_attributes_to_retrieve(meilisearch_sdk::search::Selectors::Some(&["id"]))
            .execute::<MeilisearchPostId>()
            .await?
    };

    let results = search_results;
    let total_hits = results.estimated_total_hits.unwrap_or(0) as u64;
    let post_ids = results.hits.into_iter().map(|hit| hit.result.id).collect();

    Ok((post_ids, total_hits))
}
