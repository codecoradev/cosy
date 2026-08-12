# Checklist

Tips/checklist carousel slide

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1350px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `title` | text | Yes | 100 | Wraps at 30 chars |
| `item1` | text | No | 80 |  |
| `item2` | text | No | 80 |  |
| `item3` | text | No | 80 |  |
| `item4` | text | No | 80 |  |
| `item5` | text | No | 80 |  |

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
cosy render --template checklist --data input.json --output output.png
```
