use mofumofu_dto::posts::{
    CreatePostRequest, GetPostBySlugRequest, GetPostFeedRequest, GetPostsRequest, PostAuthor,
    PostFeedResponse, PostIdPath, PostListResponse, PostResponse, PostSortOrder, UpdatePostRequest,
    UploadPostImageRequest, UploadPostImageResponse,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::create_post::create_post,
        super::get_post::get_post,
        super::get_post_by_slug::get_post_by_slug,
        super::get_post_feed::get_post_feed,
        super::get_posts::get_posts,
        super::update_post::update_post,
        super::delete_post::delete_post,
        super::upload_post_image::upload_post_image,
        super::increment_view::increment_view,
    ),
    components(
        schemas(
            PostAuthor,
            PostResponse,
            PostListResponse,
            PostFeedResponse,
            PostSortOrder,
            CreatePostRequest,
            GetPostBySlugRequest,
            GetPostFeedRequest,
            GetPostsRequest,
            UpdatePostRequest,
            UploadPostImageRequest,
            UploadPostImageResponse,
            PostIdPath,
        )
    ),
    tags(
        (name = "Posts", description = "Post endpoints")
    )
)]
pub struct PostsApiDoc;
