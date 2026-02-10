# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.4.4] - 2026-02-10

### Added

- Post write editor with split-view layout (editor left, live preview right)
- Markdown processing pipeline matching server's unified setup (14 plugins: remark-parse, remark-gfm, remark-breaks, remark-math, remark-emoji, remark-github-blockquote-alert, remark-rehype, rehype-raw, rehype-katex, rehype-highlight, rehype-slug, tocPlugin, rehype-sanitize, rehype-stringify)
- Markdown utility split into `processor.ts`, `toc.ts`, and `sanitize-schema.ts` mirroring server structure
- Write toolbar with formatting buttons (H1-H4, bold, italic, strikethrough, quote, link, image upload, math, code)
- Image upload in toolbar with 8MB limit via `uploadPostImage` API
- Collapsible header section with title input (80 char limit) and tag input using shadcn TagsInput component (max 8 tags)
- Resizable split-pane with drag handle for desktop, editor/preview toggle switch for mobile
- Live markdown preview with dynamic highlight.js theme switching (night-owl dark, atom-one-light light)
- Publish dialog with thumbnail upload (FileDropZone, 4MB), title, URL slug (auto-generated), tags, and summary inputs
- Valibot post validation schema (title 1-80, content 1+, slug 1-80 with URL-safe regex, summary 0-500, tags max 8)
- Draft API integration (`GET/POST /v0/drafts`, `PATCH/DELETE /v0/drafts/:id`, `POST /v0/drafts/:id/publish`)
- Auto-save drafts every 5 minutes with manual save button
- Draft loading from URL parameter (`/write?draft=id`)
- Server-side auth guard redirecting unauthenticated users to sign-in
- Fullscreen write layout (no navbar)
- Global error page (`+error.svelte`)
- Draft type exports (`DraftResponse`, `DraftListResponse`, `UpdateDraftRequest`, `PublishDraftRequest`)

## [2.4.0] - 2026-02-10

### Added

- Settings dialog (modal) replacing the `/settings` route
- Responsive layout: 2-column modal on desktop (lg+), fullscreen 2-step navigation on mobile
- Appearance settings page with Light/Dark/System theme selection via mode-watcher
- Account settings page with profile image upload (ImageCropper integration), display name, bio, email change, password change, and danger zone (account deletion)
- Security settings page with TOTP 2FA setup/disable/regenerate backup codes and OAuth connection management (Google, GitHub)
- Category-based sidebar navigation (Account, Security, Appearance) with auth-aware filtering
- Escape key handling: closes dialog on desktop, navigates back on mobile content view
- Body scroll lock when settings dialog is open

### Changed

- Navbar settings button now opens the settings dialog instead of navigating to `/settings`
- Both authenticated (dropdown) and unauthenticated settings buttons updated to use `onOpenSettings` callback
- Settings dialog mounted in `(main)/+layout.svelte`

## [2.0.0] - 2025-02-10

### Changed

- Move favicon from Svelte asset import to static `app.html` for faster initial load
- Add Pretendard Variable font loading via CDN with non-blocking strategy in `app.html`
- Remove dynamic favicon injection from `+layout.svelte`
