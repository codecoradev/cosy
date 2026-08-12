# Comparison

Before/after comparison

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1350px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `title` | text | Yes | 80 | Wraps at 25 chars |
| `before_label` | text | No | 30 |  |
| `before_text` | text | No | 150 | Wraps at 30 chars |
| `after_label` | text | No | 30 |  |
| `after_text` | text | No | 150 | Wraps at 30 chars |

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
cosy render --template comparison --data input.json --output output.png
```
