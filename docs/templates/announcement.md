# Announcement

Product/news announcement

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1200px |
| Height | 630px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `tag` | text | No | 30 |  |
| `headline` | text | Yes | 150 | Wraps at 35 chars |
| `detail` | text | No | 200 | Wraps at 50 chars |
| `date` | text | No | 30 |  |

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
cosy render --template announcement --data input.json --output output.png
```
