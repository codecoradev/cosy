# Testimonial

Customer testimonial card

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1200px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `quote` | text | Yes | 300 | Wraps at 38 chars |
| `author_name` | text | Yes | 50 |  |
| `author_role` | text | No | 50 |  |
| `author_company` | text | No | 50 |  |

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
      "quote": "Sample quote",
      "author_name": "Sample author name"
    }
  ]
}
```

```bash
cosy render --template testimonial --data input.json --output output.png
```
