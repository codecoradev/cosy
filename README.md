<div align="center">

# 🎨 Cosy

**Content Easy** — Lightning-fast template-based image generation in Rust

Render carousel, OG images, and marketing visuals from JSON + SVG templates at ~50ms/slide. No browser. No Chromium. Single ~5MB binary.

</div>

---

## Why Cosy?

| | Canva | Bannerbear | **Cosy** |
|---|---|---|---|
| Speed | Manual | 2-5s/render | **~50ms/render** |
| Dependency | Browser/App | Browser-based | **None (pure Rust)** |
| Binary Size | N/A | N/A | **~5MB** |
| Scriptable | No | Yes (API) | **Yes (CLI + API)** |
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

# Render single image
cosy --template og-image --data input.json --output cover.png

# Render carousel (6 slides)
cosy --template carousel-6 --data brand.json --output ./slides/
```

## Template Structure

```
templates/
├── carousel-6/
│   ├── template.svg          # SVG with {{ tokens }}
│   └── schema.json           # Field definitions
├── og-image/
│   ├── template.svg
│   └── schema.json
```

## License

Business Source License 1.1 (BSL-1.1). See [LICENSE](LICENSE).

Copyright © 2026 PT Azfirazka Digital Kreatif
