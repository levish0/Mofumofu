use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "oauth_provider")]
pub enum OAuthProvider {
    #[sea_orm(string_value = "google")]
    Google,
    #[sea_orm(string_value = "github")]
    Github,
    #[sea_orm(string_value = "discord")]
    Discord,
    #[sea_orm(string_value = "x")]
    X,
    #[sea_orm(string_value = "microsoft")]
    Microsoft,
}
