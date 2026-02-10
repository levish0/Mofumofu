use mofumofu_dto::posts::{
    CreatePostRequest, GetPostsRequest, PostIdPath, PostListResponse, PostResponse,
    UpdatePostRequest, UploadPostImageRequest, UploadPostImageResponse,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::create_post::create_post,
        super::get_post::get_post,
        super::get_posts::get_posts,
        super::update_post::update_post,
        super::delete_post::delete_post,
        super::upload_post_image::upload_post_image,
        super::increment_view::increment_view,
    ),
    components(
        schemas(
            PostResponse,
            PostListResponse,
            CreatePostRequest,
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
