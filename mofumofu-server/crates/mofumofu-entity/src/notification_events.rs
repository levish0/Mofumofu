use sea_orm::prelude::IpNetwork;
use sea_orm::prelude::*;
use uuid::Uuid;

use super::comments::Entity as CommentsEntity;
use super::common::{NotificationTargetKind, NotificationType};
use super::notification_deliveries::Entity as NotificationDeliveriesEntity;
use super::posts::Entity as PostsEntity;
use super::users::Entity as UsersEntity;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "notification_events")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(nullable)]
    pub actor_id: Option<Uuid>,
    #[sea_orm(nullable)]
    pub actor_ip: Option<IpNetwork>,
    #[sea_orm(not_null)]
    pub notification_type: NotificationType,
    #[sea_orm(column_type = "Text", not_null)]
    pub action: String,
    #[sea_orm(not_null)]
    pub target_kind: NotificationTargetKind,
    #[sea_orm(nullable)]
    pub post_id: Option<Uuid>,
    #[sea_orm(nullable)]
    pub comment_id: Option<Uuid>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub additional_data: Option<Json>,
    #[sea_orm(column_type = "TimestampWithTimeZone", not_null)]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "UsersEntity",
        from = "Column::ActorId",
        to = "super::users::Column::Id",
        on_delete = "SetNull"
    )]
    Actor,
    #[sea_orm(
        belongs_to = "PostsEntity",
        from = "Column::PostId",
        to = "super::posts::Column::Id",
        on_delete = "Cascade"
    )]
    Post,
    #[sea_orm(
        belongs_to = "CommentsEntity",
        from = "Column::CommentId",
        to = "super::comments::Column::Id",
        on_delete = "Cascade"
    )]
    Comment,
    #[sea_orm(has_many = "NotificationDeliveriesEntity")]
    Deliveries,
}

impl Related<UsersEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Actor.def()
    }
}

impl Related<PostsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<CommentsEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Comment.def()
    }
}

impl Related<NotificationDeliveriesEntity> for Entity {
    fn to() -> RelationDef {
        Relation::Deliveries.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
