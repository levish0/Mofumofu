use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct FollowRequest {
    pub followee_id: Uuid,
}
