# Matrix Display

Color-coded comparison matrix

## Preview

![Matrix Display](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/matrix-display.png)

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
| `col1` | text | ✅ | 15 |
| `col2` | text | ✅ | 15 |
| `col3` | text | ✅ | 15 |
| `r1_label` | text | ✅ | 20 |
| `r1c1` | text | ✅ | 8 |
| `r1c2` | text | ✅ | 8 |
| `r1c3` | text | ✅ | 8 |
| `r2_label` | text | ✅ | 20 |
| `r2c1` | text | ✅ | 8 |
| `r2c2` | text | ✅ | 8 |
| `r2c3` | text | ✅ | 8 |
| `r3_label` | text | ✅ | 20 |
| `r3c1` | text | ✅ | 8 |
| `r3c2` | text | ✅ | 8 |
| `r3c3` | text | ✅ | 8 |

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
cosy render --template matrix-display --data input.json --output output.png
```
