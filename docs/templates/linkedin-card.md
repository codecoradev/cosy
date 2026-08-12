# LinkedIn Card

Professional share card for LinkedIn

![LinkedIn Card sample](/samples/linkedin-card.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1200px |
| Height | 627px |
| Aspect | 1200:627 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `headline` | text | Yes | 200 | Primary headline text |
| `body` | text | No | 300 | Longer body text or description |
| `author` | text | No | 50 | Author or attributed person |
| `role` | text | No | 50 | Author's role/title |

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
      "body": "Consistent branding across every post without manual design work.",
      "author": "Linus Torvalds",
      "role": "Founder"
    }
  ]
}
```

## Render

```bash
cosy render --template linkedin-card --data input.json --output output.png
```
