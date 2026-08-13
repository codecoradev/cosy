# Benchmark Result

Performance benchmark results card

## Preview

![Benchmark Result](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/benchmark-result.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1080px
- **Aspect Ratio:** 1080:1080

## Fonts

- Inter
- JetBrains Mono
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | ✅ | 40 |
| `b1_name` | text | ✅ | 20 |
| `b1_ops` | text | ✅ | 15 |
| `b1_bar` | text | ✅ | 5 |
| `b2_name` | text | ✅ | 20 |
| `b2_ops` | text | ✅ | 15 |
| `b2_bar` | text | ✅ | 5 |
| `b3_name` | text | ✅ | 20 |
| `b3_ops` | text | ✅ | 15 |
| `b3_bar` | text | ✅ | 5 |
| `winner` | text | — | 20 |

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
cosy render --template benchmark-result --data input.json --output output.png
```
