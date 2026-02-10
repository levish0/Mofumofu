use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct FollowUserItem {
    pub id: Uuid,
    pub handle: String,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    pub followed_at: DateTime<Utc>,
}
