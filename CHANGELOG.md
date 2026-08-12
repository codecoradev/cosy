# Changelog

All notable changes to Cosy will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-08-12

### Added

- **Bearer token authentication** for HTTP API server.
  - `--token` CLI flag or `COSY_API_KEY` env var.
  - When set, all endpoints except `/api/health` require `Authorization: Bearer <token>` header.
  - When unset, auth is disabled (development mode).
  - Health endpoint reports `auth_enabled` status.
- **Docker support** — multi-stage build, multi-arch (amd64 + arm64).
  - `Dockerfile` — Rust 1.96 builder → Debian bookworm-slim runtime.
  - `docker-compose.yml` — one-command deployment with `.env` config.
  - `.env.example` — template environment configuration.
  - Health check built into Docker image.
- **Docker publish workflow** — automated GHCR image publishing on tag push.
- **CI pipeline** — 6 parallel jobs (check, fmt, clippy, test, build, verify-templates).
- **Cora AI review** — automated code review on every PR.
- **PR checks** — branch naming, conventional commits, PR body validation.
- **CLA check** — contributor license agreement verification.
- **Pre-commit hook** — adaptive language detection (Rust/TS/Go/Python) + cora review.
- **Comprehensive test suite** — 123 tests across 9 suites.
  - Unit tests: schema validation, template loading, text layout, custom filters.
  - CLI integration tests: render + validate commands.
  - API integration tests: health, templates, render, auth, CORS, error handling.
- **Mutation testing config** — `cargo-mutants.toml` for local mutation testing (100% score).

### Changed

- Release profile optimized: `codegen-units = 1`, `panic = "abort"` for smaller binary.
- `server::run()` signature now accepts `api_key: Option<String>` parameter.

### Template Features

- 18 built-in templates with gradient backgrounds, image support, and custom fonts.
- Templates: achievement-unlocked, before-after, carousel-default, comparison-table,
  crypto-price, feature-highlight, git-diff, github-profile, gradient-card, grid-gallery,
  headshot-frame, instagram-story, infographic, link-preview, quote-card, stat-card,
  tech-stack, testimonial-card.
- Bundled fonts: Inter (R/B/SB), JetBrains Mono R, Space Grotesk (Med/SB/B).
