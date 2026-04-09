use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole {
    #[sea_orm(string_value = "mod")]
    Mod,
    #[sea_orm(string_value = "admin")]
    Admin,
}

impl UserRole {
    pub fn priority(&self) -> u8 {
        match self {
            UserRole::Mod => 1,
            UserRole::Admin => 2,
        }
    }
}
