# Architecture: Cosy

*Companion to PRD `prd.md`. Tech choices live here, not in the PRD.*

## Overview

Cosy is a single-binary CLI tool written in Rust. The pipeline is:

```
JSON input → schema validation → minijinja token replacement → SVG → resvg → PNG file(s)
```

No browser, no Node.js, no external services. All rendering happens in-process via Rust crates.

## Tech Stack (Verified)

| Component | Crate | Version | Status |
|-----------|-------|---------|--------|
| SVG → PNG render | `resvg` | 0.48.1 | ✅ 21.7M downloads, production-ready |
| Template engine | `minijinja` | latest | ✅ Jinja2-compatible, lightweight |
| JSON serialization | `serde` + `serde_json` | 1.0 | ✅ Industry standard |
| Text wrapping | `textwrap` | 0.16 | ✅ Word-boundary wrapping |
| CLI parsing | `clap` | 4.x | ✅ Standard CLI framework |
| Font database | `fontdb` (via resvg) | bundled | ✅ Part of resvg ecosystem |
| Error handling | `anyhow` | 1.x | ✅ Ergonomic error chaining |
| Logging | `log` + `env_logger` | 0.4 | ✅ Standard |
| Base64 encoding | `base64` | 0.22 | ✅ For image embedding in SVG |
| HTTP API (Phase 4) | `axum` | 0.8 | ⏳ Future — not in MVP |

### Why Not Alternatives

- **rusttype** — DEAD. Last meaningful update 2+ years ago. Use resvg's built-in fontdb.
- **cosmic-text** — Overkill. Built for full text layout engine (terminal emulators). Cosy needs simple text wrapping, not complex layout.
- **Blitz** — Pre-alpha. Single-developer project. Watch list only.
- **Headless Chrome / Playwright** — The thing we're replacing. 2-5s/slide + 500MB Chromium dependency.
- **wkhtmltoimage** — C++, unmaintained, system dependency.

## Module Structure

```
src/
├── main.rs        # Entry point, clap dispatch
├── cli.rs         # CLI argument definitions (clap derive)
├── schema.rs      # Template/Input data types (serde structs)
├── template.rs    # Template loading, validation, minijinja setup
├── render.rs      # Core render pipeline: SVG → resvg → PNG
└── text.rs        # Text wrapping, base64 image encoding utilities
```

### Dependency Graph

```
main.rs → cli.rs
         ├── template.rs → schema.rs
         │                └── minijinja
         ├── render.rs → resvg + fontdb
         └── text.rs → textwrap + base64
```

## Data Flow

### 1. Input (JSON)

```json
{
  "brand": {
    "logo_url": "assets/logo.png",
    "brand_name": "CodeCora",
    "primary_color": "#f5e0dc",
    "secondary_color": "#cba6f7",
    "font_heading": "Inter",
    "font_body": "Inter"
  },
  "slides": [
    {
      "background": "solid:#1e1e2e",
      "eyebrow": "CASE STUDY",
      "headline": "How Rust cut render time by 98%",
      "body": "From 5 seconds to 50ms per slide..."
    }
  ]
}
```

### 2. Schema Validation (`template.rs`)

- Load `templates/{name}/schema.json`
- Deserialize into `TemplateDef` struct
- Validate input data against schema field definitions
- Apply brand field defaults (template defaults overridden by user input)
- Return `ValidatedData` or descriptive error

### 3. Token Replacement (`minijinja`)

- Load `templates/{name}/template.svg.j2`
- Create minijinja environment, add template
- Render with `ValidatedData` as context
- Output: complete SVG string with all tokens resolved

### 4. SVG → PNG (`render.rs`)

- Parse SVG string via `usvg` (resvg's parser)
- Load fonts via `fontdb` (bundled + user `--font-dir`)
- Render via `resvg` at configured scale (default 2x)
- Write PNG to output path

### 5. Multi-slide

- Steps 2-4 repeat for each slide in `slides[]` array
- Output files: `slide-01.png`, `slide-02.png`, ...
- Slides render sequentially (resvg is CPU-bound, parallelization adds complexity for minimal gain at 50ms/slide)

## Template Format

Each template is a directory:

```
templates/
├── carousel-default/
│   ├── schema.json         # Field definitions + defaults
│   ├── template.svg.j2     # SVG with minijinja tokens
│   └── defaults.json       # Sample data (optional, for testing)
├── social-quote/
│   ├── schema.json
│   ├── template.svg.j2
│   └── defaults.json
└── og-image/
    └── ...
```

### schema.json Structure

```json
{
  "name": "carousel-default",
  "description": "Standard 5-slide carousel with brand header",
  "canvas": {
    "width": 1080,
    "height": 1350
  },
  "brand_fields": {
    "logo_url": { "type": "string", "required": false, "default": null },
    "brand_name": { "type": "string", "required": true },
    "primary_color": { "type": "color", "required": true, "default": "#1e1e2e" },
    "secondary_color": { "type": "color", "required": true, "default": "#cba6f7" },
    "font_heading": { "type": "string", "required": false, "default": "Inter" },
    "font_body": { "type": "string", "required": false, "default": "Inter" }
  },
  "slide_fields": {
    "background": { "type": "background", "required": false, "default": "solid:#1e1e2e" },
    "eyebrow": { "type": "string", "required": false },
    "headline": { "type": "text", "required": true, "max_chars": 120 },
    "body": { "type": "text", "required": false, "max_chars": 500 },
    "cta": { "type": "string", "required": false }
  }
}
```

### template.svg.j2 Example

```xml
<svg width="{{ canvas.width }}" height="{{ canvas.height }}" xmlns="http://www.w3.org/2000/svg">
  <!-- Background -->
  <rect width="100%" height="100%" fill="{{ slide.background_color | default(brand.primary_color) }}"/>

  <!-- Brand header -->
  <text x="60" y="80" font-family="{{ brand.font_body }}" font-size="28" fill="{{ brand.secondary_color }}">
    {{ brand.brand_name }}
  </text>

  {% if slide.eyebrow %}
  <text x="60" y="200" font-family="{{ brand.font_heading }}" font-size="32" font-weight="bold"
        fill="{{ brand.secondary_color }}" letter-spacing="2">
    {{ slide.eyebrow }}
  </text>
  {% endif %}

  <!-- Headline -->
  <text x="60" y="300" font-family="{{ brand.font_heading }}" font-size="72" font-weight="bold"
        fill="#ffffff">
    {{ slide.headline | wordwrap(width=20) }}
  </text>

  <!-- Body -->
  {% if slide.body %}
  <text x="60" y="500" font-family="{{ brand.font_body }}" font-size="36" fill="#cdd6f4">
    {{ slide.body | wordwrap(width=35) }}
  </text>
  {% endif %}

  <!-- Logo -->
  {% if brand.logo_url %}
  <image href="{{ brand.logo_url | b64 }}" x="60" y="1250" width="80" height="80"/>
  {% endif %}
</svg>
```

## Font Strategy

- **Bundled**: Inter Regular, Inter Bold, JetBrains Mono Regular
- **Format**: TTF (TrueType), embedded via `include_bytes!`
- **Registration**: `fontdb` database initialized at startup, bundled fonts registered first, then user `--font-dir` fonts
- **Fallback**: If SVG references unregistered font → Inter Regular (logged warning)
- **License**: Inter = SIL Open Font License 1.1, JetBrains Mono = Apache 2.0. Both permit bundling + commercial use.

## Performance Characteristics

| Metric | Target | Basis |
|--------|--------|-------|
| Single slide render | <60ms | resvg parse + render |
| 5-slide carousel | <500ms | 5 × 60ms + overhead |
| Binary size | ~5MB | resvg + fonts + minijinja |
| Memory per render | <50MB | SVG parse + PNG buffer |
| Startup time | <10ms | single binary, no JIT |

vs. Current Playwright pipeline: 2-5 seconds per slide, 500MB+ Chromium dependency.

## Error Handling

- **anyhow** for all error propagation — ergonomic `?` operator, context chaining
- Error messages are user-facing: name the field, slide index, and what was expected
- Exit codes: 0 = success, 1 = validation error, 2 = render error
- Logging via `log` crate — `RUST_LOG=cosy=debug` for verbose output

## Testing Strategy

- **Unit tests**: per module (schema validation, text wrapping, minijinja rendering)
- **Integration tests**: full pipeline — JSON → PNG output, visual comparison
- **Snapshot tests**: reference PNGs for each template, diff on CI
- **Template tests**: each shipped template has `defaults.json` → renders without error

## CI/CD

- GitHub Actions: `cargo build`, `cargo test`, `cargo clippy` on push
- Cross-compile targets: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin`, `x86_64-pc-windows-msvc`
- Binary releases via GitHub Releases (Phase 2+)

## Phase Roadmap

| Phase | Scope | Timeline |
|-------|-------|----------|
| **Phase 1 — PoC** | Core pipeline: minijinja + resvg + 1 template | 2-3 days |
| **Phase 2 — CLI MVP** | Full CLI, 3 templates, font management | 5-7 days total |
| **Phase 3 — Internal dogfooding** | Replace Playwright for CodeCora marketing | 2-4 weeks |
| **Phase 4 — SaaS API** | axum HTTP server, rate limiting, billing | Post-validation |
| **Phase 5 — Marketplace** | Template marketplace, creator economy | Post-revenue |

## Known Risks & Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Font rendering mismatch vs Playwright | Medium | High | Visual snapshot tests, bundle exact same Inter version |
| SVG gradient limitations in resvg | Low | Medium | Template uses only `linearGradient` (supported). Mesh = unsupported, documented. |
| Text wrapping edge cases (emoji, mixed scripts) | Medium | Low | Character-limit fallback, test with Indonesian + emoji content |
| resvg API breaking changes | Low | Medium | Pin version 0.48.x, test before upgrade |
| Template complexity ceiling | Medium | Low | SVG is expressive enough for 95% of marketing visuals. Complex layouts → Phosphor/imagemagick post-processing. |

## AI Agent Integration (Phase 3)

Cosy is designed for AI agent consumption from day 1:

```bash
# AI agent generates JSON, pipes to Cosy
echo '{
  "template": "social-quote",
  "brand": {"brand_name": "CodeCora", "primary_color": "#1e1e2e"},
  "slide": {"headline": "Ship faster with Rust", "body": "Single binary, zero dependencies"}
}' | cosy render --stdin --output post.png
```

**Why this matters**: Hermes CMO and other AI agents can auto-generate marketing visuals as part of content pipelines. No API key, no network call, no rate limit. Just a subprocess call.

Phase 4 SaaS API wraps the same logic in an axum HTTP endpoint for non-local usage.
