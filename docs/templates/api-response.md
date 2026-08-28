# API Response

API JSON response viewer card

## Preview

![API Response](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/api-response.png)

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
| `status_code` | text | ✅ | 10 |
| `key1` | text | ✅ | 25 |
| `value1` | text | ✅ | 30 |
| `key2` | text | ✅ | 25 |
| `value2` | text | ✅ | 30 |
| `key3` | text | — | 25 |
| `value3` | text | — | 30 |
| `key4` | text | — | 25 |
| `value4` | text | — | 30 |

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
cosy render --template api-response --data input.json --output output.png
```
