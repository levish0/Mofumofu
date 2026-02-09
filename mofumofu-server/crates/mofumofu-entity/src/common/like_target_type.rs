use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "like_target_type")]
pub enum LikeTargetType {
    #[sea_orm(string_value = "post")]
    Post,
    #[sea_orm(string_value = "comment")]
    Comment,
}
