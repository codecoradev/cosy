# Retro Badge

Retro-style circular badge with bold text and star

## Preview

![Retro Badge](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/retro-badge.png)

## Dimensions

- **Width:** 600px
- **Height:** 600px
- **Aspect Ratio:** 600:600

## Fonts

- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `main_text` | text | ✅ | 30 |
| `sub_text` | text | — | 40 |
| `year` | text | — | 10 |

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
cosy render --template retro-badge --data input.json --output output.png
```
