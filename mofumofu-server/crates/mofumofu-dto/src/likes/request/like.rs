use mofumofu_entity::common::LikeTargetType;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct LikeRequest {
    pub target_type: LikeTargetType,
    pub target_id: Uuid,
}
