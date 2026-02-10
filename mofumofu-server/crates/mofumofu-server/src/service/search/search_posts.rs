use crate::connection::MeilisearchClient;
use crate::utils::r2_url::build_r2_public_url;
use chrono::DateTime;
use mofumofu_dto::search::{
    PostSearchItem, PostSortField, SearchPostsRequest, SearchPostsResponse, SortOrder,
};
use mofumofu_errors::errors::{Errors, ServiceResult};
use serde::Deserialize;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct IndexedPost {
    id: String,
    user_id: String,
    author_handle: String,
    author_display_name: String,
    author_profile_image: Option<String>,
    title: String,
    slug: String,
    summary: Option<String>,
    thumbnail_image: Option<String>,
    hashtags: Vec<String>,
    like_count: i32,
    comment_count: i32,
    view_count: i32,
    published_at: Option<i64>,
    created_at: i64,
}

fn build_post_sort_string(sort_by: &PostSortField, sort_order: &SortOrder) -> String {
    let field = match sort_by {
        PostSortField::CreatedAt => "created_at",
        PostSortField::LikeCount => "like_count",
        PostSortField::ViewCount => "view_count",
        PostSortField::CommentCount => "comment_count",
    };
    let order = match sort_order {
        SortOrder::Asc => "asc",
        SortOrder::Desc => "desc",
    };
    format!("{}:{}", field, order)
}

pub async fn service_search_posts(
    client: &MeilisearchClient,
    request: &SearchPostsRequest,
) -> ServiceResult<SearchPostsResponse> {
    let query = request.query.as_deref().unwrap_or("");
    info!(
        "Searching posts: query='{}', page={}, page_size={}, sort_by={:?}, sort_order={:?}",
        query, request.page, request.page_size, request.sort_by, request.sort_order
    );

    let index = client.get_client().index("posts");
    let mut search_query = index.search();

    search_query.with_query(query);
    search_query.with_page(request.page as usize);
    search_query.with_hits_per_page(request.page_size as usize);

    // Build filter: always require published posts
    let mut filters: Vec<String> = vec!["published_at IS NOT NULL".to_string()];

    if let Some(user_id) = &request.user_id {
        filters.push(format!("user_id = \"{}\"", user_id));
    }

    if let Some(after) = request.published_at_after {
        filters.push(format!("published_at >= {}", after.timestamp()));
    }

    let filter_str = filters.join(" AND ");
    search_query.with_filter(&filter_str);

    // Build sort (declared outside block so borrow lives long enough)
    let sort_str = request.sort_by.as_ref().map(|sort_by| {
        let sort_order = request.sort_order.unwrap_or(SortOrder::Desc);
        build_post_sort_string(sort_by, &sort_order)
    });
    let sort_arr: Vec<&str> = sort_str.iter().map(|s| s.as_str()).collect();
    if !sort_arr.is_empty() {
        search_query.with_sort(&sort_arr);
    }

    let results = search_query.execute::<IndexedPost>().await.map_err(|e| {
        tracing::error!("MeiliSearch post search failed: {}", e);
        Errors::MeiliSearchQueryFailed
    })?;

    let total_hits = results.total_hits.unwrap_or(0) as u64;
    let total_pages = results.total_pages.unwrap_or(0) as u32;

    let posts: Vec<PostSearchItem> = results
        .hits
        .into_iter()
        .filter_map(|hit| {
            let p = hit.result;
            let id = match Uuid::parse_str(&p.id) {
                Ok(id) => id,
                Err(e) => {
                    warn!(
                        "Invalid UUID in post search index: '{}', error: {}",
                        p.id, e
                    );
                    return None;
                }
            };
            let user_id = match Uuid::parse_str(&p.user_id) {
                Ok(id) => id,
                Err(e) => {
                    warn!(
                        "Invalid user UUID in post search index: '{}', error: {}",
                        p.user_id, e
                    );
                    return None;
                }
            };

            let created_at = DateTime::from_timestamp(p.created_at, 0)?;
            let published_at = p
                .published_at
                .and_then(|ts| DateTime::from_timestamp(ts, 0));

            Some(PostSearchItem {
                id,
                user_id,
                author_handle: p.author_handle,
                author_display_name: p.author_display_name,
                author_profile_image: p.author_profile_image.as_deref().map(build_r2_public_url),
                title: p.title,
                slug: p.slug,
                summary: p.summary,
                thumbnail_image: p.thumbnail_image,
                hashtags: p.hashtags,
                like_count: p.like_count,
                comment_count: p.comment_count,
                view_count: p.view_count,
                published_at,
                created_at,
            })
        })
        .collect();

    Ok(SearchPostsResponse {
        posts,
        page: request.page,
        page_size: request.page_size,
        total_hits,
        total_pages,
    })
}
