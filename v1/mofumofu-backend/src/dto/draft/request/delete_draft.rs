use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, IntoParams, ToSchema)]
pub struct DeleteDraftRequest {
    pub draft_id: Uuid,
}
