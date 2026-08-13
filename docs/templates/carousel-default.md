# Carousel Default

5-slide carousel with eyebrow, headline, body, CTA per slide

## Preview

![Carousel Default](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/carousel-default.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1350px
- **Aspect Ratio:** 1080:1350

## Fonts

- Inter

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `eyebrow` | text | — | 50 |
| `headline` | text | ✅ | 200 |
| `body` | text | — | 400 |

## Brand Fields

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `brand_name` | text | ✅ | — |
| `brand_handle` | text | — | — |
| `bg_color` | bg | — | #1e1e2e |
| `bg_color_end` | bg | — | #181825 |
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
cosy render --template carousel-default --data input.json --output output.png
```
