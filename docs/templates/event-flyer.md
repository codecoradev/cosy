# Event Flyer

Event flyer with date, location, and speaker info

## Preview

![Event Flyer](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/event-flyer.png)

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
| `event_name` | text | ✅ | 80 |
| `date` | text | ✅ | 40 |
| `time` | text | — | 30 |
| `location` | text | ✅ | 60 |
| `speaker` | text | — | 50 |
| `description` | text | — | 200 |

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
cosy render --template event-flyer --data input.json --output output.png
```
