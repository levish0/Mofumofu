use crate::entity::common::OAuthProvider;
use sea_orm::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_oauth_connections")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: OAuthProvider,
    pub provider_user_id: String,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::entity::users::Entity",
        from = "Column::UserId",
        to = "crate::entity::users::Column::Id",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<crate::entity::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
