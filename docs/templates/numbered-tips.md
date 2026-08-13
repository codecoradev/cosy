# Numbered Tips

Numbered list card with large numbers, ideal for tips and steps

## Preview

![Numbered Tips](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/numbered-tips.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1350px
- **Aspect Ratio:** 1080:1350

## Fonts

- Inter
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | ✅ | 100 |
| `tip1` | text | ✅ | 120 |
| `tip2` | text | ✅ | 120 |
| `tip3` | text | ✅ | 120 |
| `tip4` | text | — | 120 |
| `tip5` | text | — | 120 |

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
cosy render --template numbered-tips --data input.json --output output.png
```
