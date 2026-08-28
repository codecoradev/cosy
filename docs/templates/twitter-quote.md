# Twitter/X Quote

Quote card for X/Twitter

## Preview

![Twitter/X Quote](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/twitter-quote.png)

## Dimensions

- **Width:** 1200px
- **Height:** 675px
- **Aspect Ratio:** 1200:675

## Fonts

- Inter
- JetBrains Mono

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `quote` | text | ✅ | 280 |
| `author` | text | ✅ | 50 |
| `handle` | text | — | 30 |

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
cosy render --template twitter-quote --data input.json --output output.png
```
