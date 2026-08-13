# Feature List

Product feature list with checkmarks and descriptions

## Preview

![Feature List](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/feature-list.png)

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
| `title` | text | ✅ | 60 |
| `subtitle` | text | — | 80 |
| `f1_title` | text | ✅ | 40 |
| `f1_desc` | text | — | 60 |
| `f2_title` | text | ✅ | 40 |
| `f2_desc` | text | — | 60 |
| `f3_title` | text | ✅ | 40 |
| `f3_desc` | text | — | 60 |
| `f4_title` | text | — | 40 |
| `f4_desc` | text | — | 60 |
| `f5_title` | text | — | 40 |
| `f5_desc` | text | — | 60 |

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
cosy render --template feature-list --data input.json --output output.png
```
