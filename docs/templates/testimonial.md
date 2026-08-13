# Testimonial

Customer testimonial card

## Preview

![Testimonial](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/testimonial.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1200px
- **Aspect Ratio:** 1080:1200

## Fonts

- Inter
- JetBrains Mono

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `quote` | text | ✅ | 300 |
| `author_name` | text | ✅ | 50 |
| `author_role` | text | — | 50 |
| `author_company` | text | — | 50 |

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
cosy render --template testimonial --data input.json --output output.png
```
