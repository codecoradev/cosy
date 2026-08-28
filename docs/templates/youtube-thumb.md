# YouTube Thumbnail

Eye-catching YouTube thumbnail

## Preview

![YouTube Thumbnail](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/youtube-thumb.png)

## Dimensions

- **Width:** 1280px
- **Height:** 720px
- **Aspect Ratio:** 1280:720

## Fonts

- Inter
- JetBrains Mono

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | ✅ | 120 |
| `episode` | text | — | 20 |
| `tag` | text | — | 30 |

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
cosy render --template youtube-thumb --data input.json --output output.png
```
