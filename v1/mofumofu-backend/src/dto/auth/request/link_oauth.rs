use crate::entity::common::OAuthProvider;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct LinkOAuthRequest {
    pub provider: OAuthProvider,
    #[validate(length(min = 1, message = "Code is required."))]
    pub code: String,
}
