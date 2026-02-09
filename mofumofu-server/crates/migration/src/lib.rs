pub use sea_orm_migration::prelude::*;

mod common;
mod m20250825_033639_users;
mod m20250825_033642_create_posts;
mod m20250825_033645_oauth_providers;
mod m20250825_033646_oauth_connections;
mod m20250825_033647_create_user_role_enum;
mod m20250825_033648_create_user_roles;
mod m20251215_034351_action_resource_type_enum;
mod m20251215_034415_create_action_logs;
mod m20260209_171633_create_hashtags;
mod m20260209_171657_create_drafts;
mod m20260209_171657_create_post_hashtags;
mod m20260209_171658_create_comments;
mod m20260209_171658_create_follows;
mod m20260209_171658_create_like_target_type_enum;
mod m20260209_171658_create_likes;
mod m20260209_171659_create_report_target_type_enum;
mod m20260209_171659_create_report_status_enum;
mod m20260209_171659_create_reports;
mod m20260209_171659_create_user_bans;
mod m20260209_171659_create_moderation_resource_type_enum;
mod m20260209_171700_create_moderation_logs;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250825_033639_users::Migration),
            Box::new(m20250825_033642_create_posts::Migration),
            Box::new(m20250825_033645_oauth_providers::Migration),
            Box::new(m20250825_033646_oauth_connections::Migration),
            Box::new(m20250825_033647_create_user_role_enum::Migration),
            Box::new(m20250825_033648_create_user_roles::Migration),
            Box::new(m20251215_034351_action_resource_type_enum::Migration),
            Box::new(m20251215_034415_create_action_logs::Migration),
            Box::new(m20260209_171633_create_hashtags::Migration),
            Box::new(m20260209_171657_create_drafts::Migration),
            Box::new(m20260209_171657_create_post_hashtags::Migration),
            Box::new(m20260209_171658_create_comments::Migration),
            Box::new(m20260209_171658_create_follows::Migration),
            Box::new(m20260209_171658_create_like_target_type_enum::Migration),
            Box::new(m20260209_171658_create_likes::Migration),
            Box::new(m20260209_171659_create_report_target_type_enum::Migration),
            Box::new(m20260209_171659_create_report_status_enum::Migration),
            Box::new(m20260209_171659_create_reports::Migration),
            Box::new(m20260209_171659_create_user_bans::Migration),
            Box::new(m20260209_171659_create_moderation_resource_type_enum::Migration),
            Box::new(m20260209_171700_create_moderation_logs::Migration),
        ]
    }
}
