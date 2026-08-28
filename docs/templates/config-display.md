# Config Display

Configuration file key-value display

## Preview

![Config Display](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/config-display.png)

## Dimensions

- **Width:** 1200px
- **Height:** 630px
- **Aspect Ratio:** 1200:630

## Fonts

- Inter
- JetBrains Mono
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | ✅ | 40 |
| `k1` | text | ✅ | 25 |
| `v1` | text | ✅ | 30 |
| `k2` | text | ✅ | 25 |
| `v2` | text | ✅ | 30 |
| `k3` | text | ✅ | 25 |
| `v3` | text | ✅ | 30 |
| `k4` | text | — | 25 |
| `v4` | text | — | 30 |
| `k5` | text | — | 25 |
| `v5` | text | — | 30 |
| `k6` | text | — | 25 |
| `v6` | text | — | 30 |

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
cosy render --template config-display --data input.json --output output.png
```
