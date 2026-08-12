# TikTok Quote

Quote overlay for TikTok/Reels

![TikTok Quote sample](/samples/tiktok-quote.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1350px |
| Aspect | 1080:1350 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `quote` | text | Yes | 200 | The quote text |
| `author` | text | Yes | 50 | Author or attributed person |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "quote": "The best time to ship was yesterday. The second best is now.",
      "author": "Linus Torvalds"
    }
  ]
}
```

## Render

```bash
cosy render --template tiktok-quote --data input.json --output output.png
```
