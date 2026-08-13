# Matrix Card

2x2 decision matrix with labeled quadrants

## Preview

![Matrix Card](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/matrix-card.png)

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
| `title` | text | ✅ | 60 |
| `axis_x` | text | ✅ | 20 |
| `axis_y` | text | ✅ | 20 |
| `q1_label` | text | ✅ | 20 |
| `q1_item` | text | — | 40 |
| `q2_label` | text | ✅ | 20 |
| `q2_item` | text | — | 40 |
| `q3_label` | text | ✅ | 20 |
| `q3_item` | text | — | 40 |
| `q4_label` | text | ✅ | 20 |
| `q4_item` | text | — | 40 |

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
cosy render --template matrix-card --data input.json --output output.png
```
