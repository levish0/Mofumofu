use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema, Debug)]
pub struct DeleteCommentLikeRequest {
    pub comment_id: Uuid,
}
