# Twitter/X Quote

Quote card for X/Twitter

![Twitter/X Quote sample](/samples/twitter-quote.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1200px |
| Height | 675px |
| Aspect | 1200:675 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `quote` | text | Yes | 280 | The quote text |
| `author` | text | Yes | 50 | Author or attributed person |
| `handle` | text | No | 30 | Social media handle (e.g. "@codecoradev") |

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
      "handle": "@codecoradev"
    }
  ]
}
```

## Render

```bash
cosy render --template twitter-quote --data input.json --output output.png
```
