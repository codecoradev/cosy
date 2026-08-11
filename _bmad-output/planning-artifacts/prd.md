---
title: Cosy
created: 2026-08-11
updated: 2026-08-11
status: draft
---

# PRD: Cosy
*Content Easy — Lightning-fast template-based image generation in Rust.*

## 0. Document Purpose

This PRD defines the scope, requirements, and success metrics for **Cosy** (Content Easy), a Rust-native template-based image generation tool. It is written for the builder (ajianaz), downstream architecture workflows, and future contributors. The document follows BMAD methodology — features grouped with FRs nested (globally numbered), assumptions tagged inline and indexed.

This PRD builds on:
- Rust ecosystem research report (`research/rust-image-template-ecosystem.md`)
- Multi-agent product strategy discussion (Uteke room `disc:cosy-product-strategy`, Round 1 — CTO, CFO, CLO, COO)

## 1. Vision

Cosy replaces browser-based image rendering (Playwright + Chromium, 2-5s/slide) with a pure-Rust pipeline that renders marketing visuals — carousels, OG images, banners — from JSON + SVG templates at ~50ms/slide. No browser dependency, no Node.js runtime, no 500MB Chromium download. A single ~5MB binary.

**For CodeCora**, Cosy is the internal engine that powers all marketing visual generation — replacing the current HTML/CSS + Playwright pipeline with something 40-100x faster and deployable anywhere.

**For the market**, Cosy is a developer tool that fills the gap between "Canva is too GUI, not scriptable" and "Bannerbear/Placid are too expensive ($49-149/mo) and lock you in." It targets indie SaaS founders and dev-tool builders who need programmatic image generation without per-render SaaS pricing.

**Why it matters**: Visual content is the highest-ROI marketing channel, but generating it programmatically still means either spinning up headless browsers (slow, heavy) or paying SaaS markups for what is fundamentally SVG-to-PNG rendering. Rust makes this solvable in a single binary.

## 2. Target User

### 2.1 Jobs To Be Done

- **"As a developer-marketer**, I need to generate 50 carousel slides from a data file in under 5 seconds, so I can iterate on content without waiting for browser rendering."
- **"As an indie SaaS founder**, I need a CLI tool that generates OG images and social cards from templates, so I don't have to pay $49/mo to Bannerbear for something I could run locally."
- **"As a content creator**, I want to define my brand once (logo, colors, fonts) and apply it across templates, so every visual looks consistent without manual design work."
- **"As an AI agent (Hermes, etc.)**, I need a JSON API I can call to auto-generate marketing visuals as part of a content pipeline, so I can produce complete posts without human intervention."
- **"As a CodeCora operator**, I need to render carousel visuals for Threads/X posts programmatically, so I can scale content output without manual Canva work."

### 2.2 Non-Users (v1)

- **Non-technical marketers / designers** who need a drag-and-drop GUI. Cosy is a CLI/dev tool. Canva exists for them.
- **Enterprise teams** requiring role-based access, audit logs, collaboration features. Out of scope for MVP.
- **Mobile users.** Cosy is a desktop/server tool.
- **Users needing real-time collaborative editing.** Static render only.

### 2.3 Key User Journeys

- **UJ-1. Developer generates a carousel from a template.**
  - **Persona + context:** ajianaz, building CodeCora marketing content, has brand assets and content ready.
  - **Entry state:** Terminal open, Cosy installed, templates directory configured.
  - **Path:**
    1. Runs `cosy templates list` to see available templates.
    2. Runs `cosy render --template carousel-default --data content.json --output ./out/`.
    3. Cosy loads `templates/carousel-default/schema.json` + `template.svg.j2`.
    4. Validates `content.json` against schema (brand_fields + slide_fields).
    5. Renders each slide: minijinja token replacement → SVG → resvg → PNG.
    6. Outputs `slide-01.png` through `slide-05.png` to `./out/`.
  - **Climax:** User sees 5 PNG files generated in <500ms total. No browser, no Node.js.
  - **Resolution:** PNGs ready for upload to Threads/X/Instagram.

- **UJ-2. AI agent generates a single social card via JSON input.**
  - **Persona + context:** Hermes CMO agent, auto-generating a Threads post visual.
  - **Entry state:** Agent has content text + brand config as JSON.
  - **Path:**
    1. Agent writes JSON: `{"template": "social-quote", "brand": {...}, "slide": {"headline": "...", "body": "..."}}`.
    2. Pipes to `cosy render --stdin --output card.png`.
    3. Cosy renders single slide.
  - **Climax:** `card.png` exists, correctly styled with brand colors and fonts.
  - **Resolution:** Agent attaches PNG to social post.

- **UJ-3. Developer creates a new template.**
  - **Persona + context:** Developer/designer wants to add a "report" template for quarterly stats.
  - **Entry state:** Design mockup ready (Figma or hand-drawn).
  - **Path:**
    1. Creates `templates/report/schema.json` — defines brand_fields + slide_fields.
    2. Creates `templates/report/template.svg.j2` — SVG with minijinja tokens (`{{ brand.logo_url }}`, `{{ slide.headline }}`).
    3. Creates `templates/report/defaults.json` — sample data for testing.
    4. Runs `cosy render --template report --data defaults.json --output ./preview/`.
  - **Climax:** Preview PNG matches design mockup.
  - **Resolution:** New template available for use.

## 3. Glossary

- **Template** — A directory containing `schema.json` (field definitions), `template.svg.j2` (SVG with minijinja tokens), and optional `defaults.json` (sample data). One template = one visual style.
- **Brand Fields** — Global fields applied to all slides in a render: `logo_url`, `brand_name`, `tagline`, `url`, `primary_color`, `secondary_color`, `font_heading`, `font_body`. Defined once per template, user overrides per render.
- **Slide Fields** — Per-slide content fields defined by the template schema: `background`, `eyebrow`, `headline`, `body`, `cta`, `image_url`. Each slide in a carousel has its own set.
- **Input Data** — JSON file containing `brand_fields` + `slides[]` (or `slide` for single). Validated against template schema before rendering.
- **Render** — The process: load template → validate data → minijinja token replacement → resvg SVG-to-PNG → output file(s).
- **Slide** — A single image output (one PNG). A carousel = multiple slides; an OG image = single slide.
- **Canvas** — Fixed dimensions for output. Default: 1080×1350 (Instagram portrait/carousel). Configurable per template.

## 4. Features

### 4.1 Template Engine

**Description:** Loads template definitions, validates input data against schema, and prepares the render pipeline. A template is a directory under `./templates/` containing schema, SVG template, and optional defaults. Realizes UJ-1, UJ-3.

**Functional Requirements:**

#### FR-1: Template discovery and listing

User can list all available templates in the templates directory via `cosy templates list`. Output shows template name, description, slide count, and canvas dimensions.

**Consequences (testable):**
- `cosy templates list` returns exit code 0 and prints a formatted table.
- Templates without `schema.json` are skipped with a warning, not fatal.
- Output includes: name, description, dimensions, slide field count.

#### FR-2: Template schema validation

System validates `Input Data` JSON against the template's `schema.json` before rendering. Missing required fields produce a clear error message naming the field and slide index.

**Consequences (testable):**
- Missing required `brand_fields` → error: `"Missing required brand field: logo_url"`.
- Missing required `slide_fields` on slide index 2 → error: `"Slide 2: Missing required field: headline"`.
- Extra fields not in schema → warning, not error (forward-compatible).
- Invalid field type (e.g. string where number expected) → error with expected vs actual.

#### FR-3: Brand field defaults and override

Template defines default `Brand Fields` in `schema.json`. User `Input Data` overrides defaults. Fields not in user data fall back to template defaults.

**Consequences (testable):**
- If user provides `primary_color` but not `secondary_color`, template default for `secondary_color` is used.
- If user provides neither, both template defaults apply.
- All brand fields have sensible defaults (no hard failure if user provides nothing).

### 4.2 Rendering Pipeline

**Description:** Core SVG-to-PNG rendering using resvg. Takes validated data, runs minijinja token replacement on the SVG template, renders to PNG via resvg at 2x resolution for retina quality. Realizes UJ-1, UJ-2.

**Functional Requirements:**

#### FR-4: Multi-slide carousel rendering

User can render multiple slides from a single `Input Data` file with a `slides[]` array. Each slide produces one PNG file named `slide-{NN}.png`.

**Consequences (testable):**
- Input with 5 slides → 5 PNG files output (`slide-01.png` through `slide-05.png`).
- Slide numbering is zero-padded 2-digit.
- Total render time for 5 slides < 500ms on commodity hardware.

#### FR-5: Single-slide rendering via JSON stdin

User can render a single slide by piping JSON to stdin: `echo '{"template":"...","brand":{...},"slide":{...}}' | cosy render --stdin --output out.png`.

**Consequences (testable):**
- `--stdin` flag reads JSON from stdin instead of `--data` file.
- Single-slide mode produces exactly one PNG at the `--output` path.
- Exit code 0 on success, non-zero on render failure.

#### FR-6: SVG token replacement via minijinja

System processes the SVG template (`template.svg.j2`) through minijinja with the `Input Data` as context. Supports: variable substitution (`{{ slide.headline }}`), conditionals (`{% if slide.cta %}`), loops (`{% for item in slide.items %}`), and filters (`{{ slide.body | wordwrap(width=40) }}`).

**Consequences (testable):**
- `{{ brand.brand_name }}` in SVG → replaced with actual brand name string.
- `{% if slide.image_url %}...{% endif %}` block → included only when `image_url` present.
- `{{ slide.body | wordwrap(width=40) }}` → text wrapped at 40 chars per line.
- Unresolved variables → empty string (not error), consistent with minijinja defaults.

#### FR-7: PNG output at configurable resolution

System renders SVG to PNG via resvg at a configurable scale factor. Default: 2x (retina). Output canvas dimensions match template definition (default 1080×1350).

**Consequences (testable):**
- Default 2x → output PNG is 2160×2700 pixels.
- `--scale 1` flag → output PNG is 1080×1350 pixels.
- `--scale 3` flag → output PNG is 3240×4050 pixels.

### 4.3 Text Processing

**Description:** Pre-processes text content before SVG rendering to ensure proper layout. Wraps long text to fit within defined bounds, handles emoji and mixed scripts. Realizes UJ-1.

**Functional Requirements:**

#### FR-8: Text wrapping and layout

System wraps text to a maximum character width per line before injecting into SVG. Wrapping respects word boundaries (no mid-word breaks).

**Consequences (testable):**
- 200-character headline with `max_chars=40` → wrapped to ~5 lines, each ≤40 chars.
- Words longer than `max_chars` are broken at the character limit (last resort).
- Empty text fields → render as empty `<text>` element (no crash).

### 4.4 Font Management

**Description:** Loads and registers custom fonts (Inter, JetBrains Mono) for SVG text rendering. Fonts are bundled or user-configurable. Realizes UJ-1, UJ-3.

**Functional Requirements:**

#### FR-9: Bundled and custom font loading

System loads bundled fonts (Inter Regular/Bold, JetBrains Mono Regular) by default. User can override font directory via `--font-dir` flag or config file.

**Consequences (testable):**
- Default: bundled Inter + JetBrains Mono → no user configuration needed for CodeCora brand.
- `--font-dir /path/to/fonts` → loads all `.ttf`/`.otf` files from that directory.
- Missing font referenced in SVG → falls back to Inter Regular, logs warning.

### 4.5 CLI Interface

**Description:** Command-line interface for all operations. Uses clap for argument parsing. Designed for both human use and AI agent automation (predictable JSON output mode). Realizes UJ-1, UJ-2, UJ-3.

**Functional Requirements:**

#### FR-10: Render command

`cosy render` command accepts: `--template <name>` (required), `--data <file.json>` (required unless `--stdin`), `--output <dir|file>` (required), `--scale <n>` (optional, default 2), `--stdin` (optional flag).

**Consequences (testable):**
- All required flags missing → clear error message listing missing flags.
- `--output` pointing to non-existent directory → auto-created.
- `--output` as `.png` filename → single-slide mode (error if data has multiple slides).
- Exit code 0 on success, 1 on validation error, 2 on render error.

#### FR-11: Templates listing command

`cosy templates list` lists all templates in the templates directory. Supports `--json` flag for machine-readable output.

**Consequences (testable):**
- Default output: human-readable table (name, description, dimensions).
- `--json` output: `[{"name":"...", "description":"...", "width":1080, "height":1350, "slide_fields":["headline","body"]}, ...]`.
- Empty templates directory → message "No templates found in ./templates/".

## 5. Non-Goals (Explicit)

- **No GUI / web editor.** CLI-only for v1. Canva exists for visual editing.
- **No real-time collaboration.** Static render tool, not a design platform.
- **No template marketplace.** v1 ships with curated templates. Marketplace is a Phase 4 consideration.
- **No SaaS API / hosted service.** CLI binary only for MVP. SaaS API is Phase 4 (post-validation).
- **No animation or video output.** Static PNG only.
- **No PDF export.** PNG output only (PDF possible via external tools).
- **No image editing / manipulation.** Cosy renders from templates, not edits existing images.
- **No OCR or text extraction.** One-way: JSON → PNG.

## 6. MVP Scope

### 6.1 In Scope

- `cosy render` command with `--template`, `--data`, `--output`, `--scale`, `--stdin` flags
- `cosy templates list` command with `--json` flag
- Template engine: schema validation, brand field defaults + override
- Rendering pipeline: minijinja SVG → resvg → PNG, multi-slide + single-slide
- Text wrapping (textwrap crate)
- Bundled fonts (Inter + JetBrains Mono) + `--font-dir` override
- 3 starter templates: `carousel-default`, `social-quote`, `og-image`
- Canvas: 1080×1350 (configurable per template)
- 2x retina output by default

### 6.2 Out of Scope for MVP

- **SaaS HTTP API** (axum server) — Phase 4, post-validation. [NOTE FOR PM: Revenue path depends on this, but validating CLI first de-risks API design.]
- **Template marketplace** — Phase 4. COO: "Start with 10 curated templates, not open marketplace."
- **White-label licensing** — Phase 5+. CLO: "Long sales cycles, not worth it yet."
- **Custom font marketplace** — bundle 2 fonts, user provides their own.
- **Animation/video** — static only.
- **AI-powered template generation** — Phase 3 (AI agent integration via JSON API).
- **Web-based template editor** — not in roadmap for v1-v3.

## 7. Success Metrics

**Primary**

- **SM-1**: Render speed — 5-slide carousel renders in <500ms on commodity hardware (Oracle Cloud ARM, 4 core). Validates FR-4, FR-6, FR-7.
- **SM-2**: Internal dogfooding — Cosy replaces Playwright pipeline for all CodeCora marketing visuals within 4 weeks of MVP. Validates FR-1 through FR-11.

**Secondary**

- **SM-3**: Output quality — rendered PNGs are visually indistinguishable from current Playwright output (same layout, fonts, colors). Validates FR-6, FR-7, FR-9.
- **SM-4**: Template creation time — a new template (schema + SVG + defaults) can be created in <30 minutes by someone familiar with the format. Validates UJ-3, FR-1.

**Counter-metrics (do not optimize)**

- **SM-C1**: Binary size — do not optimize below 5MB. Correctness and font bundling matter more than binary size. Counterbalances SM-1.
- **SM-C2**: Template count — do not optimize for number of templates. 3 high-quality templates > 30 mediocre ones. Counterbalances SM-4.

## 8. Open Questions

1. **Which templates to ship first?** Current plan: `carousel-default`, `social-quote`, `og-image`. Validate against actual CodeCora marketing needs.
2. **Should `--stdin` support streaming multiple slides?** Current plan: single-slide only via stdin. If AI agents need multi-slide via stdin, revisit.
3. **Base64 image embedding vs file path?** SVG `<image href="...">` — should Cosy support base64 data URIs, file paths, or both? CTO: "base64 inflates size but works everywhere."
4. **Error output format for AI agents?** Should errors include JSON-structured output for programmatic consumption? Add `--json-errors` flag?
5. **Color palette format?** Hex (`#f5e0dc`) or named (Catppuccin Mocha `rosewater`)? Hex is universal, named is user-friendly. Plan: hex only for v1.

## 9. Assumptions Index

- [ASSUMPTION] §2.1 — AI agent integration is a real use case, not just a nice-to-have. Hermes CMO will consume Cosy via `--stdin`.
- [ASSUMPTION] §4.4 — Bundling Inter + JetBrains Mono is sufficient for v1. Users with custom fonts use `--font-dir`.
- [ASSUMPTION] §6.1 — 3 starter templates are enough to validate the product. More templates come after dogfooding.
- [ASSUMPTION] §6.2 — SaaS API is Phase 4, not MVP. CLI validation first (COO recommendation).
- [ASSUMPTION] §7 — <500ms for 5 slides is achievable. CTO estimates ~250ms (50ms/slide). Unverified.
