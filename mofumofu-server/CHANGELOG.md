# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.5.0] - 2025-02-11

### Added

#### View Count Deduplication (`mofumofu-server`)
- Redis-based view dedup: `view:{post_id}:{anonymous_user_id}` key with 1-hour TTL prevents duplicate view counts from the same visitor
- `key_exists()` and `set_with_ttl()` utilities in `redis_cache.rs`
- Cache key constants: `VIEW_DEDUP_PREFIX`, `VIEW_DEDUP_TTL_SECONDS`, `view_dedup_key()` in `mofumofu-constants`
- Best-effort: Redis failure gracefully falls through to allow the view increment

#### Follower / Following List Endpoints (`mofumofu-server`)
- `GET /v0/follows/followers` — paginated follower list with V7 cursor pagination (UUID v7)
- `GET /v0/follows/following` — paginated following list with V7 cursor pagination
- DTOs: `GetFollowersRequest`, `GetFollowingRequest`, `FollowUserItem` (id, handle, display_name, profile_image, followed_at), `FollowListResponse` (data, has_newer, has_older)
- Repository: `find_followers`, `find_following`, `exists_newer_follower`, `exists_older_follower`, `exists_newer_following`, `exists_older_following`
- Service: batch user loading with R2 public URL conversion for profile images

#### Post Feed Endpoint (`mofumofu-server`)
- `GET /v0/posts/feed` — DB-based browsable post feed with offset pagination and sort options
- `PostSortOrder` enum: `Latest` (created_at DESC), `Popular` (like_count DESC), `Oldest` (created_at ASC)
- `PostFeedItem` — lightweight flat DTO (author_handle, author_display_name, author_profile_image as top-level fields) matching `PostSearchItem` structure
- `PostFeedResponse` with `data`, `page`, `page_size`, `has_more`, `total_count`
- Separate from MeiliSearch search — uses DB directly to avoid stale counter issues

#### OAuth Profile Image Download (`mofumofu-server`)
- `download_profile_image.rs` — downloads OAuth provider profile image (Google/GitHub), validates, converts to WebP, uploads to R2
- On `complete_signup`, external profile image URL is now re-hosted on R2 instead of storing the raw external URL
- Best-effort: download/upload failure gracefully proceeds without profile image

#### Production Deployment (`deploy/`)
- `docker-compose.yml` — full production stack with bridge networking: postgres, redis-session (AOF), redis-cache (LRU), nats (JetStream), meilisearch, markdown-service, migration, server, worker
- `Caddyfile.docker` — reverse proxy config with zstd + gzip compression
- `env/*.env.example` — example environment files for postgres, meilisearch, server (50+ vars), worker
- Only the server container exposes port 8000; all inter-service communication uses Docker DNS service names

#### Search Enhancements (`mofumofu-dto`)
- `published_at_after` filter on `SearchPostsRequest` for time-window based trending queries

### Changed
- `PublicUserProfile` and `UserResponse` DTOs now include `follower_count` and `following_count` fields
- `service_increment_post_view` now requires `redis_cache` and `anonymous_user_id` parameters for dedup
- `service_complete_signup` now requires `http_client` and `r2_client` parameters for OAuth profile image re-hosting
- `service_get_post` and `service_get_post_by_slug`: `PostAuthor` now builds full R2 public URL for `profile_image` (previously returned raw storage key in some paths)
- User profile services (`get_my_profile`, `get_user_profile_by_handle`, `get_user_profile_by_id`, `update_my_profile`) updated to return follower/following counts

## [2.4.7] - 2025-02-10

### Added

#### Rendered Post Cache (`mofumofu-server`)
- **zstd-compressed render cache**: rendered HTML + TOC cached in Redis (`post:render:{post_id}`) with 30-day TTL, compressed via zstd level 3
- `CachedPostRender` struct in `mofumofu-dto` for cache serialization
- `delete_key` utility in `redis_cache.rs` for cache invalidation
- Cache key constants: `POST_RENDER_PREFIX`, `POST_RENDER_CACHE_TTL_SECONDS`, `post_render_key()` in `mofumofu-constants`

#### Post Post-Commit Utils (`mofumofu-server`)
- `service/posts/utils/post_commit.rs` — V7-style post-commit processing module
  - `post_process_post`: invalidate old cache → cache render → index in MeiliSearch (used by create, update, publish)
  - `post_process_post_delete`: invalidate cache → remove from MeiliSearch index (used by delete)
- All post-commit side effects (cache + search index) consolidated into single function calls

### Changed
- `service_create_post`, `service_update_post`, `service_delete_post`, `service_publish_draft`: added `redis_cache: &RedisClient` parameter for render caching
- `service_get_post`, `service_get_post_by_slug`: added `redis_cache: &RedisClient` parameter; check cache first, backfill on miss
- All API handlers updated to pass `state.redis_cache` to service functions
- **R2 public URL fix**: `profile_image` and `banner_image` now return full Cloudflare R2 URLs (`build_r2_public_url`) instead of raw storage keys — fixed in all post, comment, user, and search service files (12 files)

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
