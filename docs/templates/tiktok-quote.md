# Tiktok Quote

Quote overlay for TikTok/Reels

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1350px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `quote` | text | Yes | 200 | Wraps at 25 chars |
| `author` | text | Yes | 50 |  |

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
      "author": "Sample author"
    }
  ]
}
```

```bash
cosy render --template tiktok-quote --data input.json --output output.png
```
