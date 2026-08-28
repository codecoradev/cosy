# Log Viewer

Server log output viewer card

## Preview

![Log Viewer](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/log-viewer.png)

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
| `l1_time` | text | ✅ | 12 |
| `l1_level` | text | ✅ | 8 |
| `l1_msg` | text | ✅ | 55 |
| `l2_time` | text | ✅ | 12 |
| `l2_level` | text | ✅ | 8 |
| `l2_msg` | text | ✅ | 55 |
| `l3_time` | text | — | 12 |
| `l3_level` | text | — | 8 |
| `l3_msg` | text | — | 55 |
| `l4_time` | text | — | 12 |
| `l4_level` | text | — | 8 |
| `l4_msg` | text | — | 55 |

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
cosy render --template log-viewer --data input.json --output output.png
```
