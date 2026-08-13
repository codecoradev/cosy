# Version Comparison

Side-by-side software version comparison

## Preview

![Version Comparison](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/version-comparison.png)

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
| `v1_label` | text | ✅ | 15 |
| `v2_label` | text | ✅ | 15 |
| `f1_name` | text | ✅ | 25 |
| `f1_v1` | text | ✅ | 20 |
| `f1_v2` | text | ✅ | 20 |
| `f2_name` | text | ✅ | 25 |
| `f2_v1` | text | ✅ | 20 |
| `f2_v2` | text | ✅ | 20 |
| `f3_name` | text | ✅ | 25 |
| `f3_v1` | text | ✅ | 20 |
| `f3_v2` | text | ✅ | 20 |
| `f4_name` | text | — | 25 |
| `f4_v1` | text | — | 20 |
| `f4_v2` | text | — | 20 |

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
cosy render --template version-comparison --data input.json --output output.png
```
