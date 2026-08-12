# Social Quote



![Social Quote sample](/samples/social-quote.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1350px |
| Aspect | 1080:1350 |
| Fonts | Inter |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `quote` | text | Yes | 280 | The quote text |
| `author` | text | Yes | 50 | Author or attributed person |
| `author_title` | text | No | 50 | Title or position (e.g. "CTO") |

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
      "author": "Linus Torvalds",
      "author_title": "VP Engineering"
    }
  ]
}
```

## Render

```bash
cosy render --template social-quote --data input.json --output output.png
```
