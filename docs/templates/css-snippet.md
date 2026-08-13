# CSS Snippet

CSS code snippet card with syntax highlighting style

## Preview

![CSS Snippet](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/css-snippet.png)

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
| `title` | text | ✅ | 60 |
| `selector` | text | ✅ | 40 |
| `property1` | text | ✅ | 30 |
| `value1` | text | ✅ | 30 |
| `property2` | text | — | 30 |
| `value2` | text | — | 30 |
| `property3` | text | — | 30 |
| `value3` | text | — | 30 |

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
cosy render --template css-snippet --data input.json --output output.png
```
