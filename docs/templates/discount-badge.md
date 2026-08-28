# Discount Badge

Circular discount/promo badge for sales and offers

## Preview

![Discount Badge](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/discount-badge.png)

## Dimensions

- **Width:** 800px
- **Height:** 800px
- **Aspect Ratio:** 800:800

## Fonts

- Inter
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `discount` | text | ✅ | 15 |
| `label` | text | ✅ | 40 |
| `code` | text | — | 30 |
| `expires` | text | — | 30 |

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
cosy render --template discount-badge --data input.json --output output.png
```
