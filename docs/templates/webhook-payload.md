# Webhook Payload

Incoming webhook event payload viewer

## Preview

![Webhook Payload](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/webhook-payload.png)

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
| `event` | text | ✅ | 30 |
| `source` | text | ✅ | 30 |
| `timestamp` | text | — | 25 |
| `delivery_id` | text | — | 25 |
| `json_line1` | text | ✅ | 60 |
| `json_line2` | text | — | 60 |
| `json_line3` | text | — | 60 |
| `json_line4` | text | — | 60 |
| `json_line5` | text | — | 60 |

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
cosy render --template webhook-payload --data input.json --output output.png
```
