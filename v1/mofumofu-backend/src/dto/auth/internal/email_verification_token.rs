use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailVerificationTokenClaims {
    pub sub: Uuid, // user_id
    pub email: String,
    pub iat: i64, // issued at
    pub exp: i64, // expires at
}
