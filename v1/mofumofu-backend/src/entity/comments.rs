use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    #[sea_orm(column_type = "Text", not_null, string_len = 300)]
    pub content: String,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub post_id: Uuid,

    #[sea_orm(column_type = "Uuid", not_null)]
    pub user_id: Uuid,

    #[sea_orm(column_type = "Uuid", nullable)]
    pub parent_id: Option<Uuid>,

    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,

    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub updated_at: Option<DateTimeUtc>,

    #[sea_orm(column_type = "Boolean", not_null, default_value = "false")]
    pub is_deleted: bool,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub like_count: i32,

    #[sea_orm(column_type = "Integer", not_null, default_value = "0")]
    pub reply_count: i32,
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
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_delete = "Cascade"
    )]
    User,

    // 대댓글 관계 (자기 참조) - belongs_to만 사용
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_delete = "Cascade"
    )]
    ParentComment,

    // 하위 댓글들 (대댓글)
    #[sea_orm(has_many = "Entity", from = "Column::Id", to = "Column::ParentId")]
    ChildComments,
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

// 자기참조 관계를 위한 Related 구현
impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ParentComment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
