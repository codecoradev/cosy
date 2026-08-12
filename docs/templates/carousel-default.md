# Carousel

5-slide carousel with eyebrow, headline, body, CTA per slide

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1350px |
| Fonts | Inter |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `eyebrow` | text | No | 50 |  |
| `headline` | text | Yes | 200 | Wraps at 32 chars |
| `body` | text | No | 400 | Wraps at 45 chars |

## Example

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "bg_color": "#1e1e2e",
    "bg_color_end": "#181825",
    "accent_color": "#cba6f7",
    "accent_color_end": "#89b4fa"
  },
  "slides": [
    {
      "headline": "Sample headline"
    }
  ]
}
```

```bash
cosy render --template carousel-default --data input.json --output output.png
```
