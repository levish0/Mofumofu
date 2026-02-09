# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
