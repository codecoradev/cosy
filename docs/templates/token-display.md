# Token Display

Design token and color variable display

## Preview

![Token Display](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/token-display.png)

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
| `t1_name` | text | ✅ | 20 |
| `t1_value` | text | ✅ | 15 |
| `t1_color` | text | ✅ | 10 |
| `t2_name` | text | ✅ | 20 |
| `t2_value` | text | ✅ | 15 |
| `t2_color` | text | ✅ | 10 |
| `t3_name` | text | ✅ | 20 |
| `t3_value` | text | ✅ | 15 |
| `t3_color` | text | ✅ | 10 |
| `t4_name` | text | ✅ | 20 |
| `t4_value` | text | ✅ | 15 |
| `t4_color` | text | ✅ | 10 |

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
cosy render --template token-display --data input.json --output output.png
```
