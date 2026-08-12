# Instagram Story

Vertical story format

![Instagram Story sample](/samples/instagram-story.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1920px |
| Aspect | 1080:1920 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `headline` | text | Yes | 150 | Primary headline text |
| `subtext` | text | No | 300 | Supporting paragraph text |
| `tag` | text | No | 30 | Small label/badge (e.g. "NEW", "UPDATE") |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "headline": "Ship faster with templates",
      "subtext": "Stop designing every post from scratch.",
      "tag": "NEW"
    }
  ]
}
```

## Render

```bash
cosy render --template instagram-story --data input.json --output output.png
```
