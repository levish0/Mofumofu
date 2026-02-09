use crate::entity::common::OAuthProvider;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct UnlinkOAuthRequest {
    pub provider: OAuthProvider,
}
