# Instagram Story

Vertical story format

## Preview

![Instagram Story](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/instagram-story.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1920px
- **Aspect Ratio:** 1080:1920

## Fonts

- Inter
- JetBrains Mono

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `headline` | text | ✅ | 150 |
| `subtext` | text | — | 300 |
| `tag` | text | — | 30 |

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
cosy render --template instagram-story --data input.json --output output.png
```
