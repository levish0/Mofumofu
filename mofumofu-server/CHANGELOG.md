# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.1.0] - 2025-02-10

### Added

#### Error Layer (`mofumofu-errors`)
- `PostNotFound`, `DraftNotFound`, `CommentNotFound` error variants
- `protocol.rs`: `post`, `draft`, `comment` error code modules
- `post_handler.rs`, `draft_handler.rs`, `comment_handler.rs` error handlers (404 mapping)

#### Repository Layer (`mofumofu-server`)
- **posts/**: `create`, `find_by_id`, `get_by_id`, `find_by_user_id_and_slug`, `find_list` (cursor pagination + `PostFilter`), `update` (`PostUpdateParams`), `delete`
- **drafts/**: `create`, `find_by_id`, `get_by_id`, `find_by_user_id`, `update` (`DraftUpdateParams`), `delete`
- **hashtags/**: `find_by_name`, `find_or_create`
- **post_hashtags/**: `create`, `delete`, `find_by_post_id`
- **comments/**: `create`, `find_by_id`, `get_by_id`, `find_by_post_id` (cursor pagination), `update` (`CommentUpdateParams`), `soft_delete`
- **follows/**: `create`, `delete`, `find_by_follower_and_followee`
- **likes/**: `create`, `delete`, `find_by_user_and_target` (polymorphic via `LikeTargetType`)
- **reports/**: `create`, `find_by_id`, `get_by_id`, `find_list` (cursor pagination + `ReportFilter`), `update` (`ReportUpdateParams`), `exists/newer`, `exists/older`
- **user_roles/**: `create`, `delete`, `find`
- **user_bans/**: `create`, `delete`, `find`
- **moderation_logs/**: `create`, `find_list` (cursor pagination + `ModerationLogFilter`), `exists/newer`, `exists/older`

## [2.0.0] - 2025-02-10

### Added

#### Schema
- **Users**: Add `follower_count`, `following_count` counter cache columns
- **Posts**: Blog post table (slug, content, render, toc, counter caches, published_at)
- **Drafts**: Draft table (multiple per user, nullable slug, metadata jsonb)
- **Hashtags**: Hashtag table (unique name, usage_count cache)
- **PostHashtags**: Posts-Hashtags junction table (composite PK)
- **Comments**: Threaded comment table (parent_id self-ref, depth, soft delete, like_count)
- **Follows**: Follow table (self-follow CHECK constraint, unique pair)
- **Likes**: Polymorphic like table (post/comment via `like_target_type` enum)
- **Reports**: Report table (polymorphic target, status workflow via `report_status` enum)
- **UserRoles**: User role table (`user_role` enum: user/moderator/admin)
- **UserBans**: User ban table (nullable expires_at for permanent bans)
- **ModerationLogs**: Moderation log table (polymorphic resource via `moderation_resource_type` enum)

#### Enums
- `like_target_type`: post, comment
- `user_role`: user, moderator, admin
- `report_target_type`: user, post, comment
- `report_status`: pending, reviewing, resolved, dismissed
- `moderation_resource_type`: user, post, comment, system

#### Entity (`mofumofu-entity`)
- 11 new entities: posts, drafts, hashtags, post_hashtags, comments, follows, likes, reports, user_roles, user_bans, moderation_logs
- `common/` folder structure for enums (mirrors migration layout)
- Comments self-referencing relation (ParentComment/ChildComments)
- Linked impls for multi-FK entities (reports: ReporterLink/ResolverLink)
- Full has_many relations on Users entity

### Changed
- `user_oauth_connections.provider_user_id`: `string_len(255)` → `text()` (convention alignment)
- Enum enumeration calls: `.skip(1)` → `.filter(|v| !matches!(v, Xxx::Table))` (explicit Table exclusion)
