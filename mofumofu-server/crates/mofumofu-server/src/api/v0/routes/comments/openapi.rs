use mofumofu_dto::comments::{
    CommentIdPath, CommentListResponse, CommentResponse, CreateCommentRequest, GetCommentsRequest,
    UpdateCommentRequest,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::create_comment::create_comment,
        super::get_comments::get_comments,
        super::update_comment::update_comment,
        super::delete_comment::delete_comment,
    ),
    components(
        schemas(
            CommentResponse,
            CommentListResponse,
            CreateCommentRequest,
            GetCommentsRequest,
            UpdateCommentRequest,
            CommentIdPath,
        )
    ),
    tags(
        (name = "Comments", description = "Comment endpoints")
    )
)]
pub struct CommentsApiDoc;
