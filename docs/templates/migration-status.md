# Migration Status

Database migration progress tracker

## Preview

![Migration Status](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/migration-status.png)

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
| `m1_name` | text | ✅ | 30 |
| `m1_status` | text | ✅ | 10 |
| `m2_name` | text | ✅ | 30 |
| `m2_status` | text | ✅ | 10 |
| `m3_name` | text | ✅ | 30 |
| `m3_status` | text | ✅ | 10 |
| `m4_name` | text | — | 30 |
| `m4_status` | text | — | 10 |
| `m5_name` | text | — | 30 |
| `m5_status` | text | — | 10 |
| `m6_name` | text | — | 30 |
| `m6_status` | text | — | 10 |

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
cosy render --template migration-status --data input.json --output output.png
```
