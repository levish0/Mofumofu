# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
