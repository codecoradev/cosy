# Roadmap Timeline

Project roadmap with horizontal timeline

## Preview

![Roadmap Timeline](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/roadmap-timeline.png)

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
| `q1_label` | text | ✅ | 15 |
| `q1_items` | text | ✅ | 40 |
| `q2_label` | text | ✅ | 15 |
| `q2_items` | text | ✅ | 40 |
| `q3_label` | text | ✅ | 15 |
| `q3_items` | text | ✅ | 40 |
| `q4_label` | text | ✅ | 15 |
| `q4_items` | text | ✅ | 40 |

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
cosy render --template roadmap-timeline --data input.json --output output.png
```
