use sea_orm::{DeriveActiveEnum, EnumIter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize, Serialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "report_status")]
pub enum ReportStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "reviewing")]
    Reviewing,
    #[sea_orm(string_value = "resolved")]
    Resolved,
    #[sea_orm(string_value = "dismissed")]
    Dismissed,
}
