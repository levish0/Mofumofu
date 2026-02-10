use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCommentRequest {
    pub post_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
}
