# Pinterest Pin

Vertical pin format optimized for Pinterest, 2:3 aspect ratio

## Preview

![Pinterest Pin](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/pinterest-pin.png)

## Dimensions

- **Width:** 1000px
- **Height:** 1500px
- **Aspect Ratio:** 1000:1500

## Fonts

- Inter
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | ✅ | 120 |
| `subtitle` | text | — | 150 |
| `category` | text | — | 50 |

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
cosy render --template pinterest-pin --data input.json --output output.png
```
