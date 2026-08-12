# Carousel

5-slide carousel with eyebrow, headline, body, CTA per slide

![Carousel sample](/samples/carousel-default.png)

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
| `eyebrow` | text | No | 50 | Small text above headline (section label) |
| `headline` | text | Yes | 200 | Primary headline text |
| `body` | text | No | 400 | Longer body text or description |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "eyebrow": "Chapter 1",
      "headline": "Ship faster with templates",
      "body": "Consistent branding across every post without manual design work."
    }
  ]
}
```

## Render

```bash
cosy render --template carousel-default --data input.json --output output.png
```
