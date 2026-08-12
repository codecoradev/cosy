# Twitter Quote

Quote card for X/Twitter

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1200px |
| Height | 675px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `quote` | text | Yes | 280 | Wraps at 35 chars |
| `author` | text | Yes | 50 |  |
| `handle` | text | No | 30 |  |

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
cosy render --template twitter-quote --data input.json --output output.png
```
