use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_refresh_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Uuid", not_null)]
    pub user_id: Uuid,
    #[sea_orm(column_type = "Text", null)]
    pub ip_address: Option<String>,
    #[sea_orm(column_type = "Text", null)]
    pub user_agent: Option<String>,
    #[sea_orm(column_type = "Text", not_null)]
    pub refresh_token: String,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub expires_at: DateTimeUtc,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_type = "TimestampWithTimeZone", null)]
    pub revoked_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    Users,
}
impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
