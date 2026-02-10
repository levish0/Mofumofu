pub mod request;
pub mod response;

pub use request::{
    CreatePostRequest, GetPostsRequest, PostIdPath, UpdatePostRequest, UploadPostImageRequest,
};
pub use response::{PostListResponse, PostResponse, UploadPostImageResponse};
