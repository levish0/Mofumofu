pub mod request;
pub mod response;

pub use request::{CommentIdPath, CreateCommentRequest, GetCommentsRequest, UpdateCommentRequest};
pub use response::{CommentAuthor, CommentListResponse, CommentResponse};
