# Goal Tracker

Annual goal tracker with circular progress and milestones

## Preview

![Goal Tracker](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/goal-tracker.png)

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
| `title` | text | ✅ | 60 |
| `current` | text | ✅ | 10 |
| `target` | text | ✅ | 10 |
| `unit` | text | — | 20 |
| `goal1` | text | ✅ | 40 |
| `goal1_done` | text | ✅ | 10 |
| `goal2` | text | ✅ | 40 |
| `goal2_done` | text | ✅ | 10 |
| `goal3` | text | — | 40 |
| `goal3_done` | text | — | 10 |
| `goal4` | text | — | 40 |
| `goal4_done` | text | — | 10 |

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
cosy render --template goal-tracker --data input.json --output output.png
```
