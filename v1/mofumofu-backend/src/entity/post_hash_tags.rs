use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "post_hash_tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub post_id: Uuid,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub hash_tag_id: Uuid,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::posts::Entity",
        from = "Column::PostId",
        to = "super::posts::Column::Id",
        on_delete = "Cascade"
    )]
    Post,

    #[sea_orm(
        belongs_to = "super::hash_tags::Entity",
        from = "Column::HashTagId",
        to = "super::hash_tags::Column::Id",
        on_delete = "Cascade"
    )]
    HashTag,
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::hash_tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HashTag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
