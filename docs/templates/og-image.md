# Og Image

Open Graph image (1200×630) for blog posts and social previews

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1200px |
| Height | 630px |
| Fonts | Inter |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `title` | text | Yes | 200 | Wraps at 38 chars |
| `subtitle` | text | No | 150 | Wraps at 50 chars |
| `author` | text | No | 50 |  |
| `url` | text | No | 100 |  |

## Example

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "bg_color": "#1e1e2e",
    "bg_color_end": "#11111b",
    "accent_color": "#cba6f7",
    "accent_color_end": "#89b4fa"
  },
  "slides": [
    {
      "title": "Sample title"
    }
  ]
}
```

```bash
cosy render --template og-image --data input.json --output output.png
```
