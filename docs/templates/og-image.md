# OG Image

Open Graph image (1200×630) for blog posts and social previews

![OG Image sample](/samples/og-image.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1200px |
| Height | 630px |
| Aspect | 1200:630 |
| Fonts | Inter |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `title` | text | Yes | 200 | Main title or heading |
| `subtitle` | text | No | 150 | Secondary descriptive text |
| `author` | text | No | 50 | Author or attributed person |
| `url` | text | No | 100 | URL or domain (e.g. "codecora.dev") |

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
      "subtitle": "JSON in, PNG out \u2014 no design skills needed",
      "author": "Linus Torvalds",
      "url": "codecora.dev"
    }
  ]
}
```

## Render

```bash
cosy render --template og-image --data input.json --output output.png
```
