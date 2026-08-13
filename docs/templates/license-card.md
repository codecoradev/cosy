# License Card

Open source license information card

## Preview

![License Card](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/license-card.png)

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
| `license_name` | text | ✅ | 20 |
| `license_spdx` | text | ✅ | 15 |
| `description` | text | ✅ | 120 |
| `permissions` | text | ✅ | 60 |
| `conditions` | text | — | 60 |
| `limitations` | text | — | 60 |
| `copyright_year` | text | — | 10 |
| `copyright_holder` | text | — | 40 |

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
cosy render --template license-card --data input.json --output output.png
```
