use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema, Debug)]
pub struct UpdateCommentRequest {
    pub comment_id: Uuid,

    #[validate(length(min = 1, max = 300, message = "댓글은 1~300자 이내여야 합니다."))]
    pub content: String,
}
