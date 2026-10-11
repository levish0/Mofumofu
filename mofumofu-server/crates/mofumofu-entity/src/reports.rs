use sea_orm::prelude::*;
use uuid::Uuid;

use super::comments::{Column as CommentsColumn, Entity as CommentsEntity};
use super::common::ReportStatus;
use super::posts::{Column as PostsColumn, Entity as PostsEntity};
use super::users::{Column as UsersColumn, Entity as UsersEntity};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "reports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(not_null)]
    pub reporter_id: Uuid,
    #[sea_orm(nullable)]
    pub target_user_id: Option<Uuid>,
    #[sea_orm(nullable)]
    pub target_post_id: Option<Uuid>,
    #[sea_orm(nullable)]
    pub target_comment_id: Option<Uuid>,
    #[sea_orm(column_type = "Text", not_null)]
    pub reason: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(not_null)]
    pub status: ReportStatus,
    #[sea_orm(nullable)]
    pub resolved_by: Option<Uuid>,
    #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
    pub resolved_at: Option<DateTimeUtc>,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::ReporterId",
        to = "UsersColumn::Id",
        on_delete = "Cascade"
    )]
    Reporter,
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::TargetUserId",
        to = "UsersColumn::Id",
        on_delete = "Cascade"
    )]
    TargetUser,
    #[sea_orm(
        belongs_to = "PostsEntity",
        from = "Column::TargetPostId",
        to = "PostsColumn::Id",
        on_delete = "Cascade"
    )]
    TargetPost,
    #[sea_orm(
        belongs_to = "CommentsEntity",
        from = "Column::TargetCommentId",
        to = "CommentsColumn::Id",
        on_delete = "Cascade"
    )]
    TargetComment,
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::ResolvedBy",
        to = "UsersColumn::Id",
        on_delete = "SetNull"
    )]
    Resolver,
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Reporter.def()
    }
}

impl Related<PostsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::TargetPost.def()
    }
}

impl Related<CommentsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::TargetComment.def()
    }
}

pub struct TargetUserLink;

impl Linked for TargetUserLink {
    type FromEntity = Entity;
    type ToEntity = UsersEntity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::TargetUser.def()]
    }
}

pub struct ResolverLink;

impl Linked for ResolverLink {
    type FromEntity = Entity;
    type ToEntity = UsersEntity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::Resolver.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
