# Stat Comparison

Before/after comparison showing improvement metrics

## Preview

![Stat Comparison](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/stat-comparison.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1080px
- **Aspect Ratio:** 1080:1080

## Fonts

- Inter
- JetBrains Mono
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | ✅ | 80 |
| `before_label` | text | ✅ | 30 |
| `before_value` | text | ✅ | 20 |
| `after_label` | text | ✅ | 30 |
| `after_value` | text | ✅ | 20 |
| `improvement` | text | ✅ | 20 |
| `metric` | text | — | 40 |

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
cosy render --template stat-comparison --data input.json --output output.png
```
