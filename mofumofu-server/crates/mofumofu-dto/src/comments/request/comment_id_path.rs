use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct CommentIdPath {
    pub comment_id: Uuid,
}
