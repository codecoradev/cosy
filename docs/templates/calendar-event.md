# Calendar Event

Calendar-style event card with month header and day number

## Preview

![Calendar Event](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/calendar-event.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1080px
- **Aspect Ratio:** 1080:1080

## Fonts

- Inter
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `month` | text | ✅ | 20 |
| `day` | text | ✅ | 4 |
| `day_of_week` | text | ✅ | 20 |
| `event_name` | text | ✅ | 80 |
| `time` | text | — | 30 |
| `location` | text | — | 60 |

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
cosy render --template calendar-event --data input.json --output output.png
```
