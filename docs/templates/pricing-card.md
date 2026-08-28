# Pricing Card

Single pricing tier card with features list

## Preview

![Pricing Card](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/pricing-card.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1080px
- **Aspect Ratio:** 1080:1080

## Fonts

- Inter
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `tier_name` | text | ✅ | 30 |
| `price` | text | ✅ | 30 |
| `period` | text | — | 20 |
| `feature1` | text | ✅ | 50 |
| `feature2` | text | ✅ | 50 |
| `feature3` | text | ✅ | 50 |
| `feature4` | text | — | 50 |
| `feature5` | text | — | 50 |
| `cta_text` | text | — | 30 |

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
cosy render --template pricing-card --data input.json --output output.png
```
