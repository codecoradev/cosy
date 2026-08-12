# Announcement

Product/news announcement

![Announcement sample](/samples/announcement.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1200px |
| Height | 630px |
| Aspect | 1200:630 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `tag` | text | No | 30 | Small label/badge (e.g. "NEW", "UPDATE") |
| `headline` | text | Yes | 150 | Primary headline text |
| `detail` | text | No | 200 | Additional context or detail text |
| `date` | text | No | 30 | Date string (e.g. "25 August 2026") |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "tag": "NEW",
      "headline": "Ship faster with templates",
      "detail": "Available now on GitHub",
      "date": "25 August 2026"
    }
  ]
}
```

## Render

```bash
cosy render --template announcement --data input.json --output output.png
```
