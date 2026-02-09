# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2025-02-10

### Changed

- Move favicon from Svelte asset import to static `app.html` for faster initial load
- Add Pretendard Variable font loading via CDN with non-blocking strategy in `app.html`
- Remove dynamic favicon injection from `+layout.svelte`
