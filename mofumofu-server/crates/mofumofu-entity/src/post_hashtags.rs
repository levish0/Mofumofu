use sea_orm::prelude::*;
use uuid::Uuid;

use super::hashtags::Entity as HashtagsEntity;
use super::posts::Entity as PostsEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "post_hashtags")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub post_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false)]
    pub hashtag_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "PostsEntity",
        from = "Column::PostId",
        to = "super::posts::Column::Id",
        on_delete = "Cascade"
    )]
    Post,
    #[sea_orm(
        belongs_to = "HashtagsEntity",
        from = "Column::HashtagId",
        to = "super::hashtags::Column::Id",
        on_delete = "Cascade"
    )]
    Hashtag,
}

impl Related<PostsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<HashtagsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Hashtag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
