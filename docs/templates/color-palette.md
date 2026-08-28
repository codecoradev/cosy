# Color Palette

Design color palette showcase with hex codes

## Preview

![Color Palette](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/color-palette.png)

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
| `palette_name` | text | ✅ | 40 |
| `color1_name` | text | ✅ | 20 |
| `color1_hex` | text | ✅ | 10 |
| `color2_name` | text | ✅ | 20 |
| `color2_hex` | text | ✅ | 10 |
| `color3_name` | text | ✅ | 20 |
| `color3_hex` | text | ✅ | 10 |
| `color4_name` | text | ✅ | 20 |
| `color4_hex` | text | ✅ | 10 |
| `color5_name` | text | ✅ | 20 |
| `color5_hex` | text | ✅ | 10 |

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
cosy render --template color-palette --data input.json --output output.png
```
