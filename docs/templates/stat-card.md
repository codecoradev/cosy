# Stat Card

Single statistic highlight, square

![Stat Card sample](/samples/stat-card.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1080px |
| Aspect | 1080:1080 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `stat_number` | text | Yes | 20 | Large statistic number (e.g. "1M+", "42", "99.9%") |
| `stat_label` | text | Yes | 150 | Label describing the statistic |
| `source` | text | No | 100 | Optional source/citation text |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "stat_number": "99.9%",
      "stat_label": "Uptime",
      "source": "Last 30 days"
    }
  ]
}
```

## Render

```bash
cosy render --template stat-card --data input.json --output output.png
```
