# YouTube Thumbnail

Eye-catching YouTube thumbnail

![YouTube Thumbnail sample](/samples/youtube-thumb.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1280px |
| Height | 720px |
| Aspect | 1280:720 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `title` | text | Yes | 120 | Main title or heading |
| `episode` | text | No | 20 | Episode identifier (e.g. "EP 01") |
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
      "title": "Why Rust?",
      "episode": "EP 01",
      "tag": "NEW"
    }
  ]
}
```

## Render

```bash
cosy render --template youtube-thumb --data input.json --output output.png
```
