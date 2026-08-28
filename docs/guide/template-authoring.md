# Template Authoring Guide

This guide covers everything you need to create custom templates for Cosy.

## Table of Contents

- [How Cosy Works](#how-cosy-works)
- [Template Structure](#template-structure)
- [Schema Reference (schema.json)](#schema-reference-schemajson)
- [SVG Template Guide (template.svg)](#svg-template-guide-templatesvg)
- [Image Support](#image-support)
- [Built-in Fonts](#built-in-fonts)
- [Step-by-Step: Create a Template](#step-by-step-create-a-template)
- [Testing & Validation](#testing--validation)
- [Tips & Patterns](#tips--patterns)

---

## How Cosy Works

```
JSON input → minijinja template rendering → SVG string → resvg rasterization → PNG
```

1. You write an SVG file with `{{ variable }}` placeholders (minijinja syntax)
2. You define a `schema.json` that describes what fields exist and their types
3. At render time, Cosy merges your JSON data into the SVG template
4. The resulting SVG is rasterized to PNG via resvg

Cosy handles text wrapping, image embedding, and font loading automatically.

---

## Template Structure

Each template is a directory inside `templates/`:

```
templates/
├── stat-card/
│   ├── schema.json      # Field definitions (required)
│   └── template.svg     # SVG with minijinja tokens (required)
├── twitter-quote/
│   ├── schema.json
│   └── template.svg
└── ...
```

**Naming convention:** lowercase, hyphenated (e.g., `stat-card`, `og-image`).

---

## Schema Reference (schema.json)

The schema defines the template's identity, dimensions, and all customizable fields.

### Full Example

```json
{
  "id": "stat-card",
  "name": "Stat Card",
  "description": "Single statistic highlight, square",
  "dimensions": {
    "width": 1080,
    "height": 1080
  },
  "fonts": ["Inter", "JetBrains Mono"],
  "brand_fields": {
    "brand_name": {
      "type": "text",
      "required": true,
      "max": 30
    },
    "bg_color": {
      "type": "bg",
      "required": false,
      "default": "#1e1e2e"
    }
  },
  "slide_fields": {
    "stat_number": {
      "type": "text",
      "required": true,
      "max": 20
    },
    "stat_label": {
      "type": "text",
      "required": true,
      "max": 150,
      "wrap_width": 25
    }
  }
}
```

### Top-Level Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | ✅ | Unique identifier (must match directory name) |
| `name` | string | ✅ | Display name |
| `description` | string | optional | Short description |
| `dimensions` | object | ✅ | `{ "width": N, "height": N }` in pixels |
| `fonts` | string[] | optional | List of font families used (for documentation) |
| `brand_fields` | object | optional | Global fields shared across all slides |
| `slide_fields` | object | optional | Per-slide content fields |

### Field Types

| Type | Purpose | Example Values |
|------|---------|----------------|
| `text` | String content | `"127%"`, `"Revenue Growth"` |
| `image` | File path to image (PNG/JPG/WebP/SVG) | `"./logo.png"` |
| `bg` | Background color (hex) | `"#1e1e2e"` |
| `number` | Numeric value | `0.15`, `42` |
| `color` | Color value (hex) | `"#cba6f7"` |

### Field Spec Properties

| Property | Type | Applies to | Description |
|----------|------|------------|-------------|
| `type` | string | all | One of: `text`, `image`, `bg`, `number`, `color` |
| `required` | boolean | all | Whether the field must be provided (default: `false`) |
| `max` | number | `text` | Maximum character count for validation |
| `wrap_width` | number | `text` | Character width for visual line wrapping in SVG |
| `options` | string[] | `text` | Enum-like allowed values |
| `default` | any | all | Default value if not provided |

### brand_fields vs slide_fields

- **brand_fields**: Global values that stay the same across all slides (brand name, colors, logo)
- **slide_fields**: Content that changes per slide (title, body text, quote)

For single-slide templates, you still use `slide_fields` — just with one slide of data.

---

## SVG Template Guide (template.svg)

The SVG file uses [minijinja](https://docs.rs/minijinja) templating syntax for dynamic content.

### Accessing Data

```svg
<!-- Brand fields -->
{{ brand.brand_name }}
{{ brand.bg_color }}

<!-- Slide fields -->
{{ slide.title }}
{{ slide.subtitle }}
```

### Defaults

Use the `default()` filter to provide fallback values:

```svg
<stop offset="0%" stop-color="{{ brand.bg_color | default('#1e1e2e') }}"/>
```

### Conditionals

```svg
{% if slide.source %}
  <text>{{ slide.source }}</text>
{% endif %}
```

### Loops (for multi-line text wrapping)

When a text field has `wrap_width` set, Cosy pre-computes wrapped lines and provides them as `{field_name}_lines`:

```svg
{% for line in stat_label_lines %}
  <tspan x="540" dy="{{ loop.cycle(0, 48) }}">{{ line }}</tspan>
{% endfor %}
```

### Gradient Backgrounds

Most templates use a two-stop linear gradient for the background:

```svg
<defs>
  <linearGradient id="bg-grad" x1="0%" y1="0%" x2="100%" y2="100%">
    <stop offset="0%" stop-color="{{ brand.bg_color | default('#1e1e2e') }}"/>
    <stop offset="100%" stop-color="{{ brand.bg_color_end | default('#11111b') }}"/>
  </linearGradient>
</defs>
<rect width="100%" height="100%" fill="url(#bg-grad)"/>
```

### Accent Gradient (for numbers, headings)

```svg
<defs>
  <linearGradient id="acc-grad" x1="0%" y1="0%" x2="0%" y2="100%">
    <stop offset="0%" stop-color="{{ brand.accent_color | default('#cba6f7') }}"/>
    <stop offset="100%" stop-color="{{ brand.accent_color_end | default('#89b4fa') }}"/>
  </linearGradient>
</defs>
```

### Radial Glow Effect

```svg
<defs>
  <radialGradient id="glow" cx="75%" cy="25%" r="55%">
    <stop offset="0%" stop-color="{{ brand.accent_color | default('#cba6f7') }}" stop-opacity="0.07"/>
    <stop offset="100%" stop-color="{{ brand.accent_color | default('#cba6f7') }}" stop-opacity="0"/>
  </radialGradient>
</defs>
<rect width="100%" height="100%" fill="url(#glow)"/>
```

### Dots Pattern (subtle texture)

```svg
<defs>
  <pattern id="dots" x="0" y="0" width="40" height="40" patternUnits="userSpaceOnUse">
    <circle cx="20" cy="20" r="1.5" fill="#ffffff" opacity="0.06"/>
  </pattern>
</defs>
<rect width="100%" height="100%" fill="url(#dots)"/>
```

### Decorative Circle (top-right accent)

```svg
<circle cx="1000" cy="80" r="180" fill="{{ brand.accent_color | default('#cba6f7') }}" opacity="0.04"/>
```

### Border Radius (4-tier system)

Cards and UI surfaces use a standardized 4-tier radius system:

| Tier | rx | Use for |
|---|---|---|
| Small | `8` | Chips, tags, small inner elements |
| Default | `12` | Standard cards, inputs, code blocks |
| Large | `20` | Feature cards, media containers |
| Pill | `30` | Full-width banners, oversized cards |

**Exception — sub-5px decorative elements.** Thin decorative bars (divider lines, accent underlines) with height or width ≤ 5px may use any `rx` up to half their thickness (e.g. a 3px divider bar may use `rx="1.5"`). These are not UI surfaces and are exempt from the tier system. Audit scripts should skip rects with `height` or `width` ≤ 5 when checking radius compliance.

---

## Image Support

Cosy supports two image types: background images and logos.

### Background Image (bg_image)

A background texture or photo rendered behind the gradient overlay.

**Layer order (bottom → top):**
1. `bg_image` (if present, with configurable opacity)
2. Gradient overlay (opacity 0.7 when bg_image present, 1.0 otherwise)
3. Glow + dots pattern
4. Text content

**SVG pattern:**

```svg
{% if bg_image_data_uri %}
<image href="{{ bg_image_data_uri }}" width="{{ width }}" height="{{ height }}"
       preserveAspectRatio="xMidYMid slice"
       opacity="{{ brand.bg_image_opacity | default(0.15) }}"/>
{% endif %}
<rect width="{{ width }}" height="{{ height }}" fill="url(#bg-grad)"
      opacity="{% if bg_image_data_uri %}0.7{% else %}1.0{% endif %}"/>
```

**Schema fields:**

```json
{
  "bg_image": {
    "type": "image",
    "required": false,
    "description": "Background image path (PNG/JPG/WebP)"
  },
  "bg_image_opacity": {
    "type": "number",
    "required": false,
    "default": 0.15,
    "description": "Opacity of background image (0.0-1.0)"
  }
}
```

**Recommended opacity values:**
- `0.15–0.25` — subtle texture
- `0.3–0.4` — clearly visible texture (recommended for organic patterns)
- `0.5+` — bold, dominant background

### Logo

Brand logo rendered at the bottom-right corner.

```svg
{% if logo_data_uri %}
<image href="{{ logo_data_uri }}" x="{{ width - 128 }}" y="{{ height - 128 }}"
       height="48" preserveAspectRatio="xMidYMid meet"/>
{% endif %}
```

**Note:** Cosy converts image paths to data URIs automatically. In the SVG template, use `bg_image_data_uri` and `logo_data_uri` (not the raw path).

---

## Built-in Fonts

Cosy bundles these fonts (embedded at compile time — no system fonts required):

| Font Family | Weights Available | Usage |
|-------------|-------------------|-------|
| **Inter** | Regular (400), SemiBold (600), Bold (700) | Body text, labels |
| **JetBrains Mono** | Regular (400) | Code snippets, numbers |
| **Space Grotesk** | Medium (500), SemiBold (600), Bold (700) | Display headings, big numbers |

**SVG usage:**

```svg
font-family="Inter,sans-serif"
font-family="Space Grotesk,sans-serif"
font-family="JetBrains Mono,monospace"
```

Always include a generic fallback (`sans-serif` or `monospace`).

---

## Step-by-Step: Create a Template

### 1. Create the directory

```bash
mkdir templates/my-template
```

### 2. Write schema.json

```json
{
  "id": "my-template",
  "name": "My Template",
  "description": "A custom template",
  "dimensions": { "width": 1080, "height": 1080 },
  "fonts": ["Inter", "Space Grotesk"],
  "brand_fields": {
    "brand_name": { "type": "text", "required": true, "max": 30 },
    "bg_color": { "type": "bg", "required": false, "default": "#1e1e2e" },
    "bg_color_end": { "type": "bg", "required": false, "default": "#11111b" },
    "accent_color": { "type": "color", "required": false, "default": "#cba6f7" },
    "accent_color_end": { "type": "color", "required": false, "default": "#89b4fa" }
  },
  "slide_fields": {
    "headline": { "type": "text", "required": true, "max": 60, "wrap_width": 30 },
    "body": { "type": "text", "required": false, "max": 200, "wrap_width": 40 }
  }
}
```

### 3. Write template.svg

```svg
<svg xmlns="http://www.w3.org/2000/svg" width="1080" height="1080" viewBox="0 0 1080 1080">
  <defs>
    <linearGradient id="bg-grad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="{{ brand.bg_color | default('#1e1e2e') }}"/>
      <stop offset="100%" stop-color="{{ brand.bg_color_end | default('#11111b') }}"/>
    </linearGradient>
    <linearGradient id="acc-grad" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" stop-color="{{ brand.accent_color | default('#cba6f7') }}"/>
      <stop offset="100%" stop-color="{{ brand.accent_color_end | default('#89b4fa') }}"/>
    </linearGradient>
  </defs>

  <rect width="1080" height="1080" fill="url(#bg-grad)"/>

  <text x="80" y="400" font-family="Space Grotesk,sans-serif" font-size="72"
        font-weight="700" fill="url(#acc-grad)">
    {{ slide.headline }}
  </text>

  <text x="80" y="500" font-family="Inter,sans-serif" font-size="32"
        font-weight="400" fill="#cdd6f4">
    {{ slide.body }}
  </text>

  <text x="80" y="1000" font-family="Inter,sans-serif" font-size="20"
        font-weight="600" fill="#7f849c">
    {{ brand.brand_name }}
  </text>
</svg>
```

### 4. Create test data

```json
{
  "brand": {
    "brand_name": "CodeCora"
  },
  "slides": [
    {
      "headline": "Hello World",
      "body": "This is my first Cosy template."
    }
  ]
}
```

### 5. Test

```bash
# Validate
cosy validate --template my-template --data test.json

# Render
cosy render --template my-template --data test.json --output output.png --scale 2
```

---

## Testing & Validation

### Validate before rendering

```bash
cosy validate --template my-template --data test.json
```

Checks:
- All required fields are present
- Text fields don't exceed `max` length
- Field values match `options` enum (if set)

### Render with debug SVG

```bash
cosy render --template my-template --data test.json --output output.png --dump-svg
```

Dumps the processed SVG to stdout — useful for debugging layout issues.

### Render via API server

```bash
# Start server
cosy serve --port 3000

# Render via HTTP
curl -o output.png -X POST http://localhost:3000/api/render \
  -H "Content-Type: application/json" \
  -d '{
    "template": "my-template",
    "scale": 2,
    "data": {
      "brand": { "brand_name": "CodeCora" },
      "slides": [{ "headline": "Hello World", "body": "Test" }]
    }
  }'
```

---

## Tips & Patterns

### Text positioning

| Alignment | Attribute |
|-----------|-----------|
| Left-aligned | `text-anchor="start"` (default) |
| Centered | `text-anchor="middle"` + `x` at canvas center |
| Right-aligned | `text-anchor="end"` |

### Common canvas sizes

| Platform | Dimensions | Template Examples |
|----------|------------|-------------------|
| Instagram square | 1080×1080 | stat-card, dev-quote |
| Instagram portrait | 1080×1350 | carousel-default, checklist |
| Instagram story | 1080×1920 | instagram-story |
| Twitter/X | 1200×675 | twitter-quote |
| LinkedIn | 1200×627 | linkedin-card |
| Open Graph | 1200×630 | og-image, announcement |
| YouTube thumbnail | 1280×720 | youtube-thumb |
| GitHub README | 800×600 | github-readme |
| TikTok | 1080×1200 | tiktok-quote |

### minijinja pitfalls

**Avoid arithmetic in `{% set %}` within loops** — it doesn't work reliably. Instead, use `loop.index`, `loop.first`, `loop.cycle()`:

```svg
{# Good: use loop helpers #}
{% for line in title_lines %}
  <tspan x="540" dy="{{ loop.cycle(0, 48) }}">{{ line }}</tspan>
{% endfor %}

{# Bad: arithmetic in set #}
{% set offset = loop.index0 * 48 %}
```

### Color palette reference (Catppuccin Mocha)

The default Cosy color scheme:

| Token | Hex | Usage |
|-------|-----|-------|
| `bg_color` | `#1e1e2e` | Base background |
| `bg_color_end` | `#11111b` | Gradient end (mantle) |
| `accent_color` | `#cba6f7` | Mauve |
| `accent_color_end` | `#89b4fa` | Blue |
| Text primary | `#cdd6f4` | Main text |
| Text secondary | `#7f849c` | Captions, source |

### Layer order cheat sheet

```
1. bg_image (bottom, opacity from field)
2. gradient rect (opacity 0.7 if bg_image, else 1.0)
3. radial glow
4. dots pattern
5. decorative circles
6. text content
7. logo (bottom-right)
```

## Watermark / Footer

All templates must render the brand watermark with an identical spec:

```svg
{% if brand.show_brand %}
<text x="{width/2}" y="{height-30}" text-anchor="middle" font-family="Inter,sans-serif" font-size="20" font-weight="600" fill="#7f849c">{{ brand.brand_name | default('') }}</text>
{% endif %}
```

Rules:
- Position: bottom-center, 30px bottom margin (never top/corners)
- Typography: Inter 20px, weight 600, fill `#7f849c`
- Default: `show_brand: false` in `defaults.json`
- `brand_handle` may be appended as `{{ brand.brand_name }} · {{ brand.brand_handle }}` when present
