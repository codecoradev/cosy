# Branding

Cosy templates are designed to be brand-agnostic. Every template accepts the same `brand` object in the JSON input, allowing you to customize colors, gradients, background images, and logos without modifying the template.

## Color System

### Catppuccin Mocha (Default)

The default palette is based on [Catppuccin Mocha](https://catppuccin.com) — a soothing dark theme:

| Field | Value | Preview |
|-------|-------|---------|
| `bg_color` | `#1e1e2e` | Base background |
| `bg_color_end` | `#11111b` | Mantle (darker) |
| `accent_color` | `#cba6f7` | Mauve (purple) |
| `accent_color_end` | `#89b4fa` | Blue |

### Custom Colors

Override any or all colors in your JSON:

```json
{
  "brand": {
    "brand_name": "My Brand",
    "bg_color": "#0f172a",
    "bg_color_end": "#020617",
    "accent_color": "#3b82f6",
    "accent_color_end": "#06b6d4"
  }
}
```

### Color Format

All colors must be valid hex codes (`#RRGGBB`). Alpha and shorthand formats are not supported.

## Background Images

Add a subtle background image behind the gradient overlay:

```json
{
  "brand": {
    "brand_name": "My Brand",
    "bg_image": "/path/to/texture.png",  // or an https:// URL
    "bg_image_opacity": 0.15,
    "bg_image_overlay": 0.7
  }
}
```

- Supported formats: PNG, JPG, WebP — as a local path **or an `https://` URL** (fetched with a 10 s timeout, a 10 MB streamed size cap, no redirects, and private-address blocking; plain `http://` URLs work in the local CLI or with `--allow-private-images`)
- `bg_image_opacity` controls how visible the image is (0.0 = invisible, 1.0 = full opacity)
- Default: `0.15` (subtle texture). For dark, photo-like artwork use around `0.55` so the art stays visible while text contrast holds
- `bg_image_overlay` controls the gradient overlay opacity on top of the image (0.0–1.0, default: `0.7`). Lower values let more of the photo show through; keep enough overlay for title contrast

## Logo

Add a brand logo to the bottom-right corner:

```json
{
  "brand": {
    "brand_name": "My Brand",
    "logo": "/path/to/logo.png"
  }
}
```

- Supported formats: PNG, SVG
- Automatically scaled to fit within the template's logo area
- Transparent PNG or SVG recommended

## Bundled Fonts

Cosy bundles the following fonts — no system fonts needed:

| Font | Weights | Usage |
|------|---------|-------|
| Inter | Regular, Bold, SemiBold | Body text, labels |
| JetBrains Mono | Regular | Numbers, code |
| Space Grotesk | Medium, SemiBold, Bold | Headings, titles |

Fonts are compiled into the binary via `include_bytes!` — zero runtime font dependencies.
