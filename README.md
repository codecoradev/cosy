<div align="center">

# 🎨 Cosy

**Content Easy** — Lightning-fast template-based image generation in Rust

Render carousel, OG images, and marketing visuals from JSON + SVG templates at ~50ms/slide. No browser. No Chromium. Single binary.

</div>

---

## Why Cosy?

| | Canva | Bannerbear | **Cosy** |
|---|---|---|---|
| Speed | Manual | 2-5s/render | **~50ms/render** |
| Dependency | Browser/App | Browser-based | **None (pure Rust)** |
| Scriptable | No | Yes (API) | **Yes (CLI + API)** |
| Self-hosted | No | No | **Yes** |
| Price | $15/mo | $49-149/mo | **TBD** |

## How It Works

```
Template (SVG + JSON schema) → minijinja token replacement → resvg render → PNG
```

1. **Define** a template: SVG file + JSON schema (brand fields + slide fields)
2. **Fill** the template: JSON data input (brand info + per-slide content)
3. **Render**: Cosy renders PNG in ~50ms per slide

## Quick Start

```bash
# Build
cargo build --release

# List templates
cosy templates

# Render single image
cosy render --template stat-card --data input.json --output cover.png --scale 2

# Output defaults to 2x (retina) — pass --scale 1 for exact template dimensions.

# Render with inline JSON
cosy render --template stat-card --json '{"brand":{"brand_name":"CodeCora"},"slides":[{"stat_number":"127%","stat_label":"Revenue Growth"}]}' --output cover.png

# Start HTTP API server (open, no auth — dev only)
cosy serve --port 3000

# Start with bearer token auth
cosy serve --port 3000 --token mysecretkey
# or: COSY_API_KEY=mysecretkey cosy serve --port 3000
```

### HTTP API

```bash
# Health check (always public)
curl http://localhost:3000/api/health

# List templates (requires auth if token is set)
curl -H "Authorization: Bearer mysecretkey" \
  http://localhost:3000/api/templates

# Render via API (requires auth if token is set)
curl -o output.png -X POST http://localhost:3000/api/render \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer mysecretkey" \
  -d '{"template":"stat-card","scale":2,"data":{"brand":{"brand_name":"CodeCora"},"slides":[{"stat_number":"127%","stat_label":"Revenue Growth"}]}}'
```

### Authentication

When `COSY_API_KEY` is set (via env var or `--token` flag), all API endpoints except `/api/health` require an `Authorization: Bearer <token>` header. Requests without a valid token receive `401 Unauthorized`. If no key is configured, auth is disabled (development mode).

### Docker

```bash
# Pull from GHCR
docker pull ghcr.io/codecoradev/cosy:latest

# Run with auth
docker run -d \
  -p 3000:3000 \
  -e COSY_API_KEY=mysecretkey \
  --name cosy \
  ghcr.io/codecoradev/cosy:latest

# Or use docker-compose
cp .env.example .env  # edit COSY_API_KEY
docker compose up -d
```

## Template Structure

```
templates/
├── stat-card/
│   ├── template.svg          # SVG with {{ tokens }}
│   └── schema.json           # Field definitions
├── twitter-quote/
│   ├── template.svg
│   └── schema.json
└── ...                       # 18 templates included
```

## Documentation

- **[Template Authoring Guide](docs/template-authoring.md)** — How to create custom templates
- **[Changelog](CHANGELOG.md)** — Release history

## Built-in Templates (18)

| Template | Size | Use Case |
|----------|------|----------|
| stat-card | 1080×1080 | Single statistic highlight |
| twitter-quote | 1200×675 | Twitter/X quote card |
| linkedin-card | 1200×627 | LinkedIn post image |
| dev-quote | 1080×1080 | Developer quote (code snippet) |
| instagram-story | 1080×1920 | Instagram/Snapchat story |
| youtube-thumb | 1280×720 | YouTube thumbnail |
| github-readme | 800×600 | GitHub README banner |
| tiktok-quote | 1080×1200 | TikTok quote card |
| newsletter-header | 1200×400 | Email newsletter header |
| podcast-cover | 1080×1080 | Podcast episode cover |
| event-banner | 1920×1080 | Event/webinar banner |
| testimonial | 1080×1350 | Customer testimonial |
| checklist | 1080×1350 | Tips/checklist carousel |
| comparison | 1080×1350 | Before/after comparison |
| announcement | 1200×630 | Product announcement |
| carousel-default | 1080×1350 | Generic carousel slide |
| og-image | 1200×630 | Open Graph meta image |
| social-quote | 1080×1080 | General social media quote |

## License

Business Source License 1.1 (BSL-1.1). See [LICENSE](LICENSE).

Copyright © 2026 PT Azfirazka Digital Kreatif
