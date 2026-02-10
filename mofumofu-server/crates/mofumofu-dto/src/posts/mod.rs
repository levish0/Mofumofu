pub mod request;
pub mod response;

pub use request::{
    CreatePostRequest, GetPostBySlugRequest, GetPostsRequest, PostIdPath, UpdatePostRequest,
    UploadPostImageRequest,
};
pub use response::{PostAuthor, PostListResponse, PostResponse, UploadPostImageResponse};
