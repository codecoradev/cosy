# Configuration

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `COSY_API_KEY` | _(empty)_ | Bearer token for API auth. If empty, auth is disabled. |

## CLI Flags

See [CLI Reference](./cli) for all flags.

## Brand Defaults

All templates use these default brand colors unless overridden in your JSON input:

| Field | Default | Description |
|-------|---------|-------------|
| `bg_color` | `#1e1e2e` | Background gradient start |
| `bg_color_end` | `#11111b` | Background gradient end |
| `accent_color` | `#cba6f7` | Accent gradient start (purple) |
| `accent_color_end` | `#89b4fa` | Accent gradient end (blue) |

These match the [Catppuccin Mocha](https://catppuccin.com) dark palette.

## Input JSON Structure

All templates expect the same top-level structure:

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand",
    "bg_color": "#1e1e2e",
    "bg_color_end": "#11111b",
    "accent_color": "#cba6f7",
    "accent_color_end": "#89b4fa",
    "bg_image": "/path/to/bg.png",
    "bg_image_opacity": 0.15,
    "logo": "/path/to/logo.png"
  },
  "slides": [
    {
      "stat_number": "123",
      "stat_label": "Users"
    }
  ]
}
```

### Brand Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `brand_name` | string | Yes | Brand name, displayed in footer/header |
| `brand_handle` | string | No | Social handle (e.g. `@codecoradev`) |
| `bg_color` | hex | No | Background gradient start |
| `bg_color_end` | hex | No | Background gradient end |
| `accent_color` | hex | No | Accent gradient start |
| `accent_color_end` | hex | No | Accent gradient end |
| `bg_image` | path | No | Background image (PNG/JPG/WebP) |
| `bg_image_opacity` | float | No | Background image opacity (0.0–1.0, default: 0.15) |
| `logo` | path | No | Brand logo (PNG/SVG), bottom-right corner |

### Slide Fields

Each template defines its own slide fields. See [Templates](/templates/) for per-template schemas.
