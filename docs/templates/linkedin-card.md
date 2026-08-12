# Linkedin Card

Professional share card for LinkedIn

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1200px |
| Height | 627px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `headline` | text | Yes | 200 | Wraps at 40 chars |
| `body` | text | No | 300 | Wraps at 55 chars |
| `author` | text | No | 50 |  |
| `role` | text | No | 50 |  |

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
      "headline": "Sample headline"
    }
  ]
}
```

```bash
cosy render --template linkedin-card --data input.json --output output.png
```
