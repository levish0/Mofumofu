use sea_orm::prelude::*;
use uuid::Uuid;

use super::comments::Entity as CommentsEntity;
use super::post_hashtags::Entity as PostHashtagsEntity;
use super::users::Entity as UsersEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub user_id: Uuid,
    #[sea_orm(column_type = "Text", not_null)]
    pub title: String,
    #[sea_orm(column_type = "Text", not_null)]
    pub slug: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub thumbnail_image: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub summary: Option<String>,
    #[sea_orm(column_type = "Text", not_null)]
    pub content: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub render: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub toc: Option<Json>,
    #[sea_orm(not_null)]
    pub like_count: i32,
    #[sea_orm(not_null)]
    pub comment_count: i32,
    #[sea_orm(not_null)]
    pub view_count: i32,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub published_at: Option<DateTimeUtc>,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    User,
    #[sea_orm(has_many = "CommentsEntity")]
    Comments,
    #[sea_orm(has_many = "PostHashtagsEntity")]
    PostHashtags,
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<CommentsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Comments.def()
    }
}

impl Related<PostHashtagsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::PostHashtags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
