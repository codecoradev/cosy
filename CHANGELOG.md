# Changelog

All notable changes to Cosy will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **GET signed-URL rendering** — `GET /r/{template}.{png|webp}?d=<base64url
  JSON>&sig=<hmac-sha256>` for `<meta property="og:image">`-style dynamic
  images: blog engines embed a plain URL, no client library, no POST. The
  signature covers `{template}:{d}` (URLs can't be transplanted across
  templates), is compared in constant time, and uses the API key as the
  signing key; the route is disabled (404) when no key is configured.
  Payload cap 8 KB → 413, bad signature → 403, cacheable response
  (`Cache-Control: public, max-age=3600`). New deps: `hmac` + `sha2`
  (pure Rust). Closes #95.
- **CSV batch rendering** — `cosy render --dataset rows.csv --out-dir out/`
  renders one image per row in parallel (rayon). Header = field names;
  number/boolean columns are type-coerced per the template schema; brand
  defaults come from the template's `defaults.json`. Optional `_filename`
  column names the output (sanitized), optional `_data` column carries a
  full JSON slide/array (multi-slide rows write a `{stem}_slides/`
  subdirectory). `--fail-fast` stops unscheduled rows after the first
  failure; exit code 1 if any row failed; `--json-output` emits a
  structured summary. Works with `--format webp`. Closes #96.
- **Metadata passthrough** — optional `metadata` (any JSON value, 4 KB cap)
  on `POST /api/render`, echoed verbatim in the JSON envelope (binary
  responses log it instead — raw image bytes can't carry it). CLI:
  `--metadata '<json>'` echoed in `--json-output`, fail-fast on malformed
  JSON. Oversized metadata → 413. Closes #97.
- **WebP output format** — `cosy render --format webp` (CLI) and
  `image_format: "webp"` on `POST /api/render`. Container choice is
  independent of `response_format`: a JSON envelope can carry PNG or WebP
  entries (`image_format` field added per slide in the envelope). Binary
  responses get the correct `Content-Type` (`image/png` / `image/webp`).
  WebP encoding is lossless via the `image` crate (no new dependencies);
  the `-o` path is used verbatim, multi-slide directory mode names files
  `NN.webp`. Closes #94.
- **Notebook-style templates** - `notebook-cover` and `notebook-page`
  (1080x1350) replicate the viral handwritten study-notes carousel format:
  ruled paper, spiral binding, dashed callout boxes, and a cloud punchline.
  Includes bundled handwritten fonts Kalam (Light/Regular/Bold) and Caveat
  (Medium/Bold), drawn inline SVG pipeline icons (no emoji), shrink-to-fit
  hero/chapter titles, and dynamic-height dashed boxes and punchline bubble.

- **Multi-slide HTTP API** — `POST /api/render` now accepts
  `response_format: "json"` to render every slide of a carousel and return them
  as base64 PNG entries with template dimensions, and `slide_index` to pick a
  specific slide with the default binary PNG response. Empty slide arrays and
  out-of-range indices are rejected with a 400.
- **Inline text markup** — `*bold*`, `_italic_`, and `*color:#hex*...*color*` in
  markup-enabled text fields (opt-in via schema `options: ["markup"]`), rendered as
  styled `<tspan>` runs with markup-aware line wrapping. Bundled Inter Italic,
  Inter Bold Italic, and Inter Black fonts so emphasis resolves to real faces.
- **`text_color`** slide field on the 9 text-quote templates — per-page text color
  for light backgrounds, validated as hex at render time (`color` field type is now
  type-checked).
- **`hashtag`** slide field on the 9 text-quote templates — accent-colored `#tag`
  line near the bottom.
- **`bg_overlay_opacity`** brand field on the 10 quote-family templates — gradient
  overlay opacity when `bg_image` is set (default `0.7`, backward compatible).

## [0.2.0] — 2026-08-28

### Added

- **148 templates** (up from 18) — social cards, code snippets, stats/dashboards, quotes, banners, docs graphics, all with `template.svg` + `schema.json` + `defaults.json`.
- **Real-case defaults** for every template, validated against each schema at render time.
- **Background image + custom logo support** with data-URI injection and overlay opacity control.
- **Watermark/brand footer** — standardized bottom-center footer across all templates, off by default (`show_brand` toggle).
- **Numeric schema fields** — percentages/measures are typed numbers driving proportional bar fills and progress rings (poll-result, progress-card, goal-tracker).
- **--stdin / --json / --json-output** CLI flags for machine-driven rendering.
- **VitePress docs site** — 148-page template gallery with MinIO-hosted previews, Pinterest-style masonry layout.
- **Test suite grown to 126 tests** (render, CLI, schema, filters), 100% mutation score maintained.

### Fixed

- **XML-escaping in the renderer** — literal `&`, `<`, `>` in user data no longer abort rendering with "malformed entity reference".
- **10 high-severity visual defects** found by a full vision audit of all 148 rendered templates (clipped/overlapping/garbled output): command-card, docker-command, tech-stack, roadmap-timeline, week-schedule, vscode-config, webhook-payload, wallpaper-quote, gradient-quote, poll-result.
- **43 medium/low findings** — vertical balance, low-contrast elements, wrong status-color semantics, broken glyphs, non-proportional bars, YAML/code indentation, duplicated prefixes in default data (`vv2.1.0`, `votes votes`, `EP EP`).
- **18 default-data corrections** — defaults now satisfy their own schema constraints and no longer duplicate template-rendered prefixes.
- **Release CI** — multi-platform binary build without cargo-zigbuild.

### Changed

- Default branch moved to `develop` (integration); `main` is the release mirror.
- Repository is now public.

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
