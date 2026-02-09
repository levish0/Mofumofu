use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "drafts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub user_id: Uuid,

    #[sea_orm(column_type = "Text", nullable, string_len = 80)]
    pub title: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub thumbnail_image: Option<String>,

    #[sea_orm(column_type = "Text", nullable, string_len = 500)]
    pub summary: Option<String>,

    #[sea_orm(column_type = "Text", nullable)]
    pub content: Option<String>,

    #[sea_orm(column_type = "Text", not_null, string_len = 80)]
    pub slug: String,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub updated_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
