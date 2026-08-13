# Book Cover

Book/ebook cover design with title and author

## Preview

![Book Cover](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/book-cover.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1620px
- **Aspect Ratio:** 1080:1620

## Fonts

- Inter
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | ✅ | 120 |
| `subtitle` | text | — | 150 |
| `author` | text | ✅ | 50 |
| `edition` | text | — | 30 |

## Brand Fields

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `brand_name` | text | ✅ | — |
| `brand_handle` | text | — | — |
| `bg_color` | bg | — | #1e1e2e |
| `bg_color_end` | bg | — | #11111b |
| `accent_color` | bg | — | #cba6f7 |
| `accent_color_end` | bg | — | #89b4fa |
| `bg_image` | image | — | — |
| `bg_image_opacity` | number | — | 0.15 |
| `logo` | image | — | — |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Ajianaz",
    "brand_handle": "@ajianaz"
  },
  "slides": [
    {
      // fill slide fields here
    }
  ]
}
```

## Usage

```bash
cosy render --template book-cover --data input.json --output output.png
```
