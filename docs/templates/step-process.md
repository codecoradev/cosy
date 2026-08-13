# Step Process

Horizontal step-by-step flow diagram with arrows

## Preview

![Step Process](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/step-process.png)

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
| `title` | text | ✅ | 80 |
| `step1_title` | text | ✅ | 30 |
| `step1_desc` | text | — | 80 |
| `step2_title` | text | ✅ | 30 |
| `step2_desc` | text | — | 80 |
| `step3_title` | text | ✅ | 30 |
| `step3_desc` | text | — | 80 |
| `step4_title` | text | — | 30 |
| `step4_desc` | text | — | 80 |

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
cosy render --template step-process --data input.json --output output.png
```
