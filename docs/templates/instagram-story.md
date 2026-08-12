# Instagram Story

Vertical story format

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1920px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `headline` | text | Yes | 150 | Wraps at 28 chars |
| `subtext` | text | No | 300 | Wraps at 40 chars |
| `tag` | text | No | 30 |  |

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
cosy render --template instagram-story --data input.json --output output.png
```
