# Youtube Thumb

Eye-catching YouTube thumbnail

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1280px |
| Height | 720px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `title` | text | Yes | 120 | Wraps at 30 chars |
| `episode` | text | No | 20 |  |
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
      "title": "Sample title"
    }
  ]
}
```

```bash
cosy render --template youtube-thumb --data input.json --output output.png
```
