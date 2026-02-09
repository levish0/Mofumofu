use crate::common::UserRole;
use crate::m20250825_033639_users::Users;
use sea_orm_migration::prelude::*;
use strum::IntoEnumIterator;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserRoles::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("uuidv7()")),
                    )
                    .col(ColumnDef::new(UserRoles::UserId).uuid().not_null())
                    .col(
                        ColumnDef::new(UserRoles::Role)
                            .enumeration(
                                UserRole::Table,
                                UserRole::iter()
                                    .filter(|v| !matches!(v, UserRole::Table))
                                    .collect::<Vec<_>>(),
                            )
                            .not_null()
                            .default(Expr::cust("'user'::user_role")),
                    )
                    .col(ColumnDef::new(UserRoles::GrantedBy).uuid().null())
                    .col(
                        ColumnDef::new(UserRoles::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_user")
                            .from(UserRoles::Table, UserRoles::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_roles_granted_by")
                            .from(UserRoles::Table, UserRoles::GrantedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique: one role per user per type
        manager
            .create_index(
                Index::create()
                    .name("uq_user_roles_user_role")
                    .table(UserRoles::Table)
                    .col(UserRoles::UserId)
                    .col(UserRoles::Role)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // Index: User's roles
        manager
            .create_index(
                Index::create()
                    .name("idx_user_roles_user_id")
                    .table(UserRoles::Table)
                    .col(UserRoles::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserRoles {
    #[sea_orm(iden = "user_roles")]
    Table,
    Id,
    UserId,
    Role,
    GrantedBy,
    CreatedAt,
}
