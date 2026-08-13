# Flame Graph

Performance profiling flame graph

## Preview

![Flame Graph](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/flame-graph.png)

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
| `f1_name` | text | ✅ | 20 |
| `f1_pct` | text | ✅ | 8 |
| `f1_width` | text | ✅ | 8 |
| `f2_name` | text | ✅ | 20 |
| `f2_pct` | text | ✅ | 8 |
| `f2_width` | text | ✅ | 8 |
| `f3_name` | text | ✅ | 20 |
| `f3_pct` | text | ✅ | 8 |
| `f3_width` | text | ✅ | 8 |
| `f4_name` | text | — | 20 |
| `f4_pct` | text | — | 8 |
| `f4_width` | text | — | 8 |
| `total_time` | text | — | 15 |

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
cosy render --template flame-graph --data input.json --output output.png
```
