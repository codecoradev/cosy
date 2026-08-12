# Quick Start

## CLI — Render an Image

Create a JSON input file:

```json
{
  "brand": {
    "brand_name": "My Brand",
    "brand_handle": "@mybrand",
    "bg_color": "#1e1e2e",
    "bg_color_end": "#11111b",
    "accent_color": "#cba6f7",
    "accent_color_end": "#89b4fa"
  },
  "slides": [
    {
      "stat_number": "1M+",
      "stat_label": "Downloads"
    }
  ]
}
```

Render:

```bash
cosy render --template stat-card --data input.json --output output.png
```

## List Templates

```bash
cosy templates
```

Output:

```
Available templates (18):
  - achievement-unlocked
  - before-after
  - carousel-default
  - comparison-table
  - crypto-price
  - feature-highlight
  - git-diff
  - github-profile
  - gradient-card
  - grid-gallery
  - headshot-frame
  - instagram-story
  - infographic
  - link-preview
  - quote-card
  - stat-card
  - tech-stack
  - testimonial-card
```

## Validate Input

Check your JSON against a template's schema before rendering:

```bash
cosy validate --template stat-card --data input.json
```

## HTTP Server

Start the server:

```bash
# Without auth (development)
cosy serve --port 3000

# With bearer token auth
COSY_API_KEY=mysecret cosy serve --port 3000
# Or
cosy serve --port 3000 --token mysecret
```

Render via API:

```bash
curl -X POST http://localhost:3000/api/render \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer mysecret" \
  -d '{
    "template": "stat-card",
    "data": {
      "brand": { "brand_name": "My Brand" },
      "slides": [{ "stat_number": "42", "stat_label": "Users" }]
    }
  }' \
  -o output.png
```

## Next Steps

- [CLI Reference](./cli) — All commands and flags
- [Templates](/templates/) — Browse all 18 templates with examples
- [Template Authoring](./template-authoring) — Create your own templates
- [HTTP Server](./server) — Full API server guide
