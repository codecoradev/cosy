# Deploy Status

Deployment pipeline status tracker

## Preview

![Deploy Status](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/deploy-status.png)

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
| `environment` | text | ✅ | 15 |
| `version` | text | ✅ | 15 |
| `s1_name` | text | ✅ | 15 |
| `s1_status` | text | ✅ | 10 |
| `s2_name` | text | ✅ | 15 |
| `s2_status` | text | ✅ | 10 |
| `s3_name` | text | ✅ | 15 |
| `s3_status` | text | ✅ | 10 |
| `s4_name` | text | ✅ | 15 |
| `s4_status` | text | ✅ | 10 |
| `commit` | text | — | 15 |

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
cosy render --template deploy-status --data input.json --output output.png
```
