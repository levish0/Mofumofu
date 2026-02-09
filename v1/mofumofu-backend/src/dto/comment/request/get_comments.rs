use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum CommentSortOrder {
    Latest,
    Oldest,
    Popular,
}

impl Default for CommentSortOrder {
    fn default() -> Self {
        Self::Latest
    }
}

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct GetCommentsRequest {
    pub post_id: Uuid,

    #[serde(default = "default_page")]
    pub page: u32,

    #[serde(default = "default_per_page")]
    pub per_page: u32,

    #[serde(default)]
    pub sort: CommentSortOrder,
}

#[derive(Deserialize, ToSchema, Debug, Validate)]
pub struct GetRepliesRequest {
    pub parent_comment_id: Uuid,

    #[serde(default = "default_page")]
    pub page: u32,

    #[serde(default = "default_per_page")]
    pub per_page: u32,

    #[serde(default)]
    pub sort: CommentSortOrder,
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    20
}
