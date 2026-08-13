# Metric Grid

2x2 grid of metrics with numbers and labels

## Preview

![Metric Grid](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/metric-grid.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1080px
- **Aspect Ratio:** 1080:1080

## Fonts

- Inter
- Space Grotesk
- JetBrains Mono

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | — | 80 |
| `metric1_value` | text | ✅ | 20 |
| `metric1_label` | text | ✅ | 50 |
| `metric2_value` | text | ✅ | 20 |
| `metric2_label` | text | ✅ | 50 |
| `metric3_value` | text | ✅ | 20 |
| `metric3_label` | text | ✅ | 50 |
| `metric4_value` | text | ✅ | 20 |
| `metric4_label` | text | ✅ | 50 |

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
cosy render --template metric-grid --data input.json --output output.png
```
