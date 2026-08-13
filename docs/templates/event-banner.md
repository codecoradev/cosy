# Event Banner

Event/webinar banner

## Preview

![Event Banner](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/event-banner.png)

## Dimensions

- **Width:** 1920px
- **Height:** 1080px
- **Aspect Ratio:** 1920:1080

## Fonts

- Inter
- JetBrains Mono

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `event_name` | text | ✅ | 100 |
| `date` | text | — | 30 |
| `time` | text | — | 20 |
| `speaker` | text | — | 50 |

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
cosy render --template event-banner --data input.json --output output.png
```
