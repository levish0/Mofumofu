use mofumofu_dto::search::{
    PostSearchItem, PostSortField, SearchPostsRequest, SearchPostsResponse, SearchUsersRequest,
    SearchUsersResponse, SortOrder, UserSearchItem,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::search_users::search_users,
        super::search_posts::search_posts,
    ),
    components(
        schemas(
            SortOrder,
            SearchUsersRequest,
            SearchUsersResponse,
            UserSearchItem,
            PostSortField,
            SearchPostsRequest,
            SearchPostsResponse,
            PostSearchItem,
        )
    ),
    tags(
        (name = "Search", description = "Search endpoints")
    )
)]
pub struct SearchApiDoc;
