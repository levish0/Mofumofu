use sea_orm::prelude::*;
use uuid::Uuid;

use super::post_hashtags::Entity as PostHashtagsEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "hashtags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text", not_null, unique)]
    pub name: String,
    #[sea_orm(not_null)]
    pub usage_count: i32,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub last_used_at: Option<DateTimeUtc>,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "PostHashtagsEntity")]
    PostHashtags,
}

impl Related<PostHashtagsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::PostHashtags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
