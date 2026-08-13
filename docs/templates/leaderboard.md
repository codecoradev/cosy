# Leaderboard

Ranked leaderboard display

## Preview

![Leaderboard](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/leaderboard.png)

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
| `title` | text | ✅ | 40 |
| `e1_name` | text | ✅ | 20 |
| `e1_score` | text | ✅ | 10 |
| `e2_name` | text | ✅ | 20 |
| `e2_score` | text | ✅ | 10 |
| `e3_name` | text | ✅ | 20 |
| `e3_score` | text | ✅ | 10 |
| `e4_name` | text | — | 20 |
| `e4_score` | text | — | 10 |
| `e5_name` | text | — | 20 |
| `e5_score` | text | — | 10 |

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
cosy render --template leaderboard --data input.json --output output.png
```
