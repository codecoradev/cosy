# Stat Card

Single statistic highlight, square

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1080px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `stat_number` | text | Yes | 20 |  |
| `stat_label` | text | Yes | 150 | Wraps at 25 chars |
| `source` | text | No | 100 |  |

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
      "stat_number": "Sample stat number",
      "stat_label": "Sample stat label"
    }
  ]
}
```

```bash
cosy render --template stat-card --data input.json --output output.png
```
