pub mod create_post;
pub mod get_post_by_slug;
pub mod get_posts;
pub mod post_id_path;
pub mod update_post;
pub mod upload_post_image;

pub use create_post::CreatePostRequest;
pub use get_post_by_slug::GetPostBySlugRequest;
pub use get_posts::GetPostsRequest;
pub use post_id_path::PostIdPath;
pub use update_post::UpdatePostRequest;
pub use upload_post_image::UploadPostImageRequest;
