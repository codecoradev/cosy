# Changelog

All notable changes to Cosy will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Docs
- Fix blank Template Authoring page (inline `{{ }}` parsed as Vue), derive the
  template count from `templates/`, WCAG-compliant brand colors in light mode,
  favicon and header logo, and fail the docs CI build on render errors (#138).

### Fixed
- Findings from a Cora scan (#136): `--stdin`/`--json` temp input is now
  unique, `O_EXCL`-created and removed afterwards (was a fixed `/tmp` name);
  empty `slides` is an error instead of a panic; remote-image DNS resolution
  has a timeout and local image reads are capped at 10 MB; the `b64` and
  `wordwrap` filters now work on unescaped text (URLs with `&` were broken,
  wrapping could split an XML entity); out-of-range numeric XML references
  are escaped; `/api/templates` returns 500 if listing fails; JSON envelope
  `width`/`height` now match the image; `Bearer` and `HTTP://` are
  case-insensitive; CLI `--scale` must be in (0, 16]; CSV UTF-8 BOM is
  stripped; `is_public_ip` rejects `2001:db8::/32` and NAT64 targets.

## [0.5.1] — 2026-10-07

### Security
- `POST /api/render` hardening (#127): `scale` limited to 0.1–4.0, at most
  20 slides per JSON request, renders bounded by a CPU-sized semaphore,
  `template` must be a plain name (no filesystem paths), and input is
  validated against the template schema like the CLI/signed GET route.
- An empty `COSY_API_KEY` (docker-compose default) now means "auth
  disabled" instead of an unusable empty secret / empty signing key.

### Added
- Signed GET URLs (#129): dedicated `--signing-key` / `COSY_SIGNING_KEY`
  (falls back to the API key) and optional signed `exp` expiry.
- `cosy serve --host` to choose the bind address; a warning is logged when
  auth is disabled on a non-loopback address.

### Performance
- Render pipeline (#128): the font database is shared as `Arc` instead of
  cloned per slide, pixel buffers are moved (not copied) into the encoder
  (`OutputFormat::encode_owned`), `template.svg` sources are cached by
  mtime, and remote images are cached for 5 minutes (32 entries / 64 MB).
- `/api/health` returns the startup template count instead of re-parsing
  every `schema.json` (restart to pick up templates added at runtime);
  `/api/templates` reads off the async workers.

### Docs
- Brand: logo symbol variants and 3-tone fan mark icon set (#125, #126).

## [0.5.0] — 2026-09-30

### Added
- **6 sosmed templates** — `tweet-screenshot`, `linkedin-text-post`,
  `testimonial-card` (SVG star rating 0–5), `audiogram-card` (episode
  hero + static waveform), `meme-text-card` (top/bottom captions +
  accent frame), `sparkline-card` (polyline + gradient area fill +
  end-dot marker). All follow the semantic-id convention. Template
  count 158.

### Fixed
- Markup fields: the generic pre-wrap loop no longer overwrites `_lines`
  with raw-marker wrapping (which dropped bold/italic emphasis and
  desynced from `_segments`).

### Added
- **Semantic element ids in SVG output** — templates wrap field content
  in `<g id="f-<field>">` (+ `bg` / `brand` / `chrome`); usvg preserves
  the ids through the SVG write path so downstream consumers can target
  individual elements. First batch: `og-image`, `code-screenshot`.
  Convention now required for new templates (documented in the
  authoring guide). Raster output unchanged (md5-identical on defaults).
  Closes #117.
- **`code-screenshot` template** — macOS editor window (traffic lights,
  title, filename + language badge) with **real syntax highlighting**:
  keywords (language-aware: Rust/Go/Python/TS/JS/Bash/SQL), strings,
  numbers, and comments tokenized per line via the new `highlight`
  module; colors on both dark (default) and light themes. 1200×675,
  lines beyond 13 clip inside the code window. Closes #93.
- **`code` schema option** — `options: ["code"]` on a text field emits
  `<field>_lines` (plain) + `<field>_segments` (colored) using the
  sibling `code_lang` field; reusable by other templates.

## [0.4.0] — 2026-09-29

### Added
- **SVG output format** — `cosy render --format svg` (CLI) and
  `image_format: "svg"` (API; also in the JSON envelope and via `.svg`
  extensions). Vector-native: emits the usvg-resolved tree with **text
  converted to paths** — fully self-contained, zero font dependencies.
  `scale` is a raster concept and is ignored (canvas stays at 1x).
  Content-Type `image/svg+xml`. Closes #111.

## [0.3.0] — 2026-09-29

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
