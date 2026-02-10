# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.4.5] - 2025-02-10

### Added

#### Worker (`mofumofu-worker`)
- **Post indexing**: `IndexPostJob` / `PostIndexAction` — indexes posts to MeiliSearch with title, summary, author, hashtags, and counter caches
- **Post reindexing**: `ReindexPostsJob` — batch reindex all posts with UUID v7 cursor pagination and self-enqueue
- Post index settings: searchable (title, summary, author_handle, author_display_name, hashtags), filterable (user_id, hashtags, published_at), sortable (created_at, like_count, view_count, comment_count), ranking rules include `sort`
- NATS streams: `INDEX_POST_STREAM`, `INDEX_POST_SUBJECT`, `INDEX_POST_CONSUMER`, `REINDEX_POSTS_STREAM`, `REINDEX_POSTS_SUBJECT`, `REINDEX_POSTS_CONSUMER`

#### Bridge (`mofumofu-server`)
- `index_post(worker, post_id)` — queue post index job
- `delete_post_from_index(worker, post_id)` — queue post deletion from index
- `start_reindex_posts(worker, batch_size)` — start full post reindex

#### DTOs (`mofumofu-dto`)
- **search/request**: `PostSortField` enum (CreatedAt, LikeCount, ViewCount, CommentCount), `SearchPostsRequest` (query, user_id, page, page_size, sort_by, sort_order)
- **search/response**: `PostSearchItem`, `SearchPostsResponse` (with IntoResponse impl)
- **posts/request**: `GetPostBySlugRequest` (handle + slug query params)

#### Service Layer (`mofumofu-server`)
- `service_search_posts` — MeiliSearch post search with filter (published_at IS NOT NULL, optional user_id), sort, and pagination
- `service_get_post_by_slug` — resolve handle → user, then user_id + slug → post with full PostResponse

#### Handler / Router Layer (`mofumofu-server`)
- `GET /v0/search/posts` — public post search endpoint
- `GET /v0/posts/by-slug?handle={handle}&slug={slug}` — slug-based post lookup

### Changed
- `service_create_post`, `service_update_post`, `service_delete_post`, `service_publish_draft`: added `worker: &WorkerClient` parameter, queue MeiliSearch index/delete jobs after transaction commit (best-effort, non-blocking)
- API handlers for create_post, update_post, delete_post, publish_draft: pass `state.worker` to service layer
- Search OpenAPI doc: added post search schemas and path

## [2.4.1] - 2025-02-10

### Added

#### Handler / Router Layer (`mofumofu-server`)
- **posts/**: `create_post` (POST), `get_post` (GET by ID), `get_posts` (GET list), `update_post` (PATCH), `delete_post` (DELETE), `upload_post_image` (POST multipart with `POST_IMAGE_MAX_SIZE` body limit), `increment_view` (POST)
- **drafts/**: `create_draft` (POST), `get_draft` (GET by ID), `get_drafts` (GET list), `update_draft` (PATCH), `delete_draft` (DELETE), `publish_draft` (POST)
- **comments/**: `create_comment` (POST), `get_comments` (GET list), `update_comment` (PATCH), `delete_comment` (DELETE)
- **likes/**: `create_like` (POST), `delete_like` (DELETE), `check_like_status` (GET)
- **follows/**: `create_follow` (POST), `delete_follow` (DELETE), `check_follow_status` (GET)
- **hashtags/**: `get_trending` (GET)
- **reports/**: `create_report` (POST), `get_reports` (GET list), `update_report` (PATCH)
- **user_roles/**: `grant_role` (POST), `revoke_role` (POST), `get_user_roles` (GET)
- **user_bans/**: `ban_user` (POST), `unban_user` (POST), `get_user_bans` (GET)
- **moderation_logs/**: `get_moderation_logs` (GET list)
- OpenAPI spec (`utoipa`) for all 10 new domain modules, merged into `V0ApiDoc`

#### DTOs (`mofumofu-dto`)
- **Path extractors**: `PostIdPath`, `DraftIdPath`, `CommentIdPath`, `ReportIdPath`, `UserIdPath` (IntoParams + ToSchema)
- **Request DTOs**: `RevokeRoleRequest`, `UnbanUserRequest`, `TrendingQuery` (limit 1-50, default 10)

### Changed
- `LikeRequest`, `FollowRequest`: added `IntoParams` derive (required for query parameter extraction in status check endpoints)
- `hashtags` module: added `request` submodule

## [2.3.1] - 2025-02-10

### Added

#### Constants (`mofumofu-constants`)
- `ModerationAction` enum (`user:ban`, `user:unban`, `user:grant_role`, `user:revoke_role`, `report:resolve`, `report:dismiss`, `post:delete`, `comment:delete`) with `as_str()`, `Display`, `FromStr`
- `POST_IMAGES_PREFIX`, `POST_IMAGE_MAX_SIZE`, `post_image_key()` storage key helpers

#### Error Layer (`mofumofu-errors`)
- `FollowSelfFollow`, `FollowAlreadyFollowing`, `FollowNotFollowing` error variants + handler
- `LikeAlreadyLiked`, `LikeNotLiked`, `LikeTargetNotFound` error variants + handler
- `CommentDepthExceeded`, `CommentParentNotFound`, `CommentPostMismatch` error variants (comment handler expanded)
- `DraftMissingTitle`, `DraftMissingContent` error variants (draft handler expanded)
- `MarkdownRenderFailed` error variant + handler
- `ReportNotFound`, `ReportAlreadyExists` handler
- `UserAlreadyBanned`, `UserNotBanned`, `UserDoesNotHaveRole`, `UserAlreadyHasRole`, `CannotManageHigherOrEqualRole` handler (user handler expanded)
- Protocol constants for follow, like, markdown, report modules

#### DTOs (`mofumofu-dto`)
- **posts/**: `CreatePostRequest`, `GetPostsRequest` (IntoParams), `UpdatePostRequest`, `PostResponse` (`from_model` with hashtag names), `PostListResponse`, `UploadPostImageRequest` (FromMultipart), `UploadPostImageResponse`
- **drafts/**: `UpdateDraftRequest`, `PublishDraftRequest`, `DraftResponse`, `DraftListResponse`
- **comments/**: `CreateCommentRequest`, `GetCommentsRequest` (IntoParams), `UpdateCommentRequest`, `CommentResponse`, `CommentListResponse`
- **likes/**: `LikeRequest`, `LikeStatusResponse`
- **follows/**: `FollowRequest`, `FollowStatusResponse`
- **hashtags/**: `HashtagResponse`, `TrendingHashtagsResponse`
- **reports/**: `CreateReportRequest`, `GetReportsRequest` (IntoParams), `UpdateReportRequest`, `ReportResponse`, `ReportListResponse`
- **user_roles/**: `GrantRoleRequest`, `UserRoleResponse`
- **user_bans/**: `BanUserRequest`, `UserBanResponse`
- **moderation_logs/**: `GetModerationLogsRequest` (IntoParams), `ModerationLogResponse`, `ModerationLogListResponse`

#### Repository (`mofumofu-server`)
- **posts/**: `increment_view_count`, `increment_like_count`, `decrement_like_count`, `increment_comment_count`, `decrement_comment_count`, `exists/newer`, `exists/older`
- **comments/**: `increment_like_count`, `decrement_like_count`, `exists/newer`, `exists/older`
- **hashtags/**: `increment_usage_count`, `decrement_usage_count`, `find_trending`
- **post_hashtags/**: `delete_all_by_post_id`
- **user/**: `increment_follower_count`, `decrement_follower_count`, `increment_following_count`, `decrement_following_count`
- **user_roles/**: `find_highest` (`repository_get_highest_user_role`)

#### Entity (`mofumofu-entity`)
- `UserRole::priority()` method for role hierarchy comparison

#### Bridge (`mofumofu-server`)
- `markdown_client::render_markdown` — calls markdown-service `/render` endpoint (camelCase API contract)

#### Config (`mofumofu-config`)
- `markdown_service_url` field in `ServerConfig`

#### Service Layer (`mofumofu-server`)
- **posts/**: `create_post`, `get_post`, `get_posts` (cursor pagination), `update_post`, `delete_post`, `increment_view` (Redis dedup), `upload_post_image` (R2)
- **drafts/**: `create_draft`, `get_draft`, `get_drafts`, `update_draft`, `delete_draft`, `publish_draft` (markdown render + hashtag management)
- **comments/**: `create_comment` (threaded with depth), `get_comments` (cursor pagination), `update_comment`, `delete_comment` (soft delete + counter decrement)
- **likes/**: `create_like`, `delete_like` (polymorphic post/comment + counter cache), `check_like_status`
- **follows/**: `create_follow`, `delete_follow` (follower/following counter cache), `check_follow_status`
- **hashtags/**: `get_trending_hashtags`
- **reports/**: `create_report`, `get_reports` (cursor pagination), `update_report` (resolve/dismiss workflow + moderation log)
- **user_roles/**: `grant_role` (priority ACL: actor vs max(target, new_role)), `revoke_role` (rows_affected check), `get_user_roles`
- **user_bans/**: `ban_user` (priority ACL), `unban_user` (priority ACL), `get_user_bans`, `check_active_ban`
- **moderation_logs/**: `get_moderation_logs` (cursor pagination)

### Changed
- All services use **transaction-first pattern** (TOCTOU safety): `conn.begin()` → reads/checks → writes → `txn.commit()`
- `repository_create_moderation_log` accepts `ModerationAction` enum instead of raw `String`
- Ownership checks use `Errors::ForbiddenError` instead of `Errors::UserUnauthorized`
- External HTTP calls (markdown render) execute before transaction to avoid holding txn during network I/O

## [2.1.1] - 2025-02-10

### Added

#### Markdown Service (`markdown-service/`)
- Bun + Elysia + unified.js markdown rendering microservice
- `POST /render` — Markdown to HTML + TOC conversion
- `GET /health` — Health check endpoint
- Pipeline: remark-parse → remark-gfm → remark-breaks → remark-math → remark-emoji → remark-github-blockquote-alert → remark-rehype → rehype-raw → rehype-katex → rehype-highlight → rehype-slug → tocPlugin → rehype-sanitize → rehype-stringify
- Supports: GFM, LaTeX (KaTeX), code syntax highlighting (highlight.js), emoji, GitHub alert blockquotes, SVG
- XSS prevention: rehype-sanitize custom schema (style attribute blocked, javascript: URI blocked, protocol whitelist)
- Dockerfile (multi-stage build)
- Prettier config + CI workflow (`markdown-service-build.yml`)

### Changed (vs V1 markdown-service)
- Switch to top-level imports (removed per-call dynamic imports for better performance)
- Remove `remark-toc` (replaced by custom tocPlugin, unnecessary dependency)
- tocItems closure variable replaced with per-call array injection (eliminates side effects)
- Return HTTP 500 on error (V1 returned 200)
- Support `PORT` environment variable (default 6700)
- Remove global `style` attribute allowance (XSS vector)
- File separation: `processor.ts`, `sanitize-schema.ts`, `toc.ts` (separation of concerns)

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
