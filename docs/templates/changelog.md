# Changelog

Release changelog card with version and categorized changes

## Preview

![Changelog](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/changelog.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1350px
- **Aspect Ratio:** 1080:1350

## Fonts

- Inter
- JetBrains Mono
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `version` | text | ✅ | 20 |
| `date` | text | — | 30 |
| `feature1` | text | ✅ | 80 |
| `feature2` | text | — | 80 |
| `feature3` | text | — | 80 |
| `fix1` | text | — | 80 |
| `fix2` | text | — | 80 |

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
cosy render --template changelog --data input.json --output output.png
```
