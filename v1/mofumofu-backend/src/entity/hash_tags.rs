use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "hash_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Text", unique, not_null, string_len = 100)]
    pub name: String,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub usage_count: i32,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub last_used_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::post_hash_tags::Entity",
        from = "Column::Id",
        to = "super::post_hash_tags::Column::HashTagId"
    )]
    PostHashTags,
}

impl Related<super::post_hash_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostHashTags.def()
    }
}

// HashTag -> Posts (Many-to-Many through PostHashTags)
#[derive(Debug, Clone)]
pub struct HashTagToPostsLink;

impl Linked for HashTagToPostsLink {
    type FromEntity = Entity;
    type ToEntity = super::posts::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::PostHashTags.def(),
            super::post_hash_tags::Relation::Post.def(),
        ]
    }
}

impl ActiveModelBehavior for ActiveModel {}
