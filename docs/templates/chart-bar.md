# Bar Chart

Bar chart visualization with multiple data points

## Preview

![Bar Chart](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/chart-bar.png)

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
| `b1_label` | text | ✅ | 20 |
| `b1_value` | text | ✅ | 10 |
| `b2_label` | text | ✅ | 20 |
| `b2_value` | text | ✅ | 10 |
| `b3_label` | text | ✅ | 20 |
| `b3_value` | text | ✅ | 10 |
| `b4_label` | text | ✅ | 20 |
| `b4_value` | text | ✅ | 10 |
| `b5_label` | text | — | 20 |
| `b5_value` | text | — | 10 |

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
cosy render --template chart-bar --data input.json --output output.png
```
