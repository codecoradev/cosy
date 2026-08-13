# Database Schema

Database table schema visualization

## Preview

![Database Schema](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/database-schema.png)

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
| `table_name` | text | ✅ | 25 |
| `c1_name` | text | ✅ | 20 |
| `c1_type` | text | ✅ | 15 |
| `c1_key` | text | — | 5 |
| `c2_name` | text | ✅ | 20 |
| `c2_type` | text | ✅ | 15 |
| `c2_key` | text | — | 5 |
| `c3_name` | text | — | 20 |
| `c3_type` | text | — | 15 |
| `c3_key` | text | — | 5 |
| `c4_name` | text | — | 20 |
| `c4_type` | text | — | 15 |
| `c4_key` | text | — | 5 |
| `c5_name` | text | — | 20 |
| `c5_type` | text | — | 15 |
| `c5_key` | text | — | 5 |

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
cosy render --template database-schema --data input.json --output output.png
```
