# Comparison Table

Side-by-side comparison table with two options

## Preview

![Comparison Table](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/comparison-table.png)

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
| `option_a_name` | text | ✅ | 30 |
| `option_b_name` | text | ✅ | 30 |
| `row1_label` | text | ✅ | 30 |
| `row1_a` | text | ✅ | 40 |
| `row1_b` | text | ✅ | 40 |
| `row2_label` | text | ✅ | 30 |
| `row2_a` | text | ✅ | 40 |
| `row2_b` | text | ✅ | 40 |
| `row3_label` | text | — | 30 |
| `row3_a` | text | — | 40 |
| `row3_b` | text | — | 40 |
| `row4_label` | text | — | 30 |
| `row4_a` | text | — | 40 |
| `row4_b` | text | — | 40 |

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
cosy render --template comparison-table --data input.json --output output.png
```
