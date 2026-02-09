use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct GetCommentByIdRequest {
    pub comment_id: Uuid,
}
