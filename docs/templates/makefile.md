# Makefile

Makefile targets viewer

## Preview

![Makefile](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/makefile.png)

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
| `title` | text | ✅ | 30 |
| `t1_name` | text | ✅ | 15 |
| `t1_desc` | text | ✅ | 40 |
| `t2_name` | text | ✅ | 15 |
| `t2_desc` | text | ✅ | 40 |
| `t3_name` | text | ✅ | 15 |
| `t3_desc` | text | ✅ | 40 |
| `t4_name` | text | — | 15 |
| `t4_desc` | text | — | 40 |
| `t5_name` | text | — | 15 |
| `t5_desc` | text | — | 40 |

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
cosy render --template makefile --data input.json --output output.png
```
