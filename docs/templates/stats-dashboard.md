# Stats Dashboard

Dashboard with multiple metric cards

## Preview

![Stats Dashboard](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/stats-dashboard.png)

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
| `title` | text | ✅ | 40 |
| `m1_label` | text | ✅ | 15 |
| `m1_value` | text | ✅ | 15 |
| `m1_delta` | text | — | 10 |
| `m2_label` | text | ✅ | 15 |
| `m2_value` | text | ✅ | 15 |
| `m2_delta` | text | — | 10 |
| `m3_label` | text | ✅ | 15 |
| `m3_value` | text | ✅ | 15 |
| `m3_delta` | text | — | 10 |
| `m4_label` | text | ✅ | 15 |
| `m4_value` | text | ✅ | 15 |
| `m4_delta` | text | — | 10 |

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
cosy render --template stats-dashboard --data input.json --output output.png
```
