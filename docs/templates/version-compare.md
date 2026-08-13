# Version Compare

Side-by-side version comparison

## Preview

![Version Compare](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/version-compare.png)

## Dimensions

- **Width:** 1200px
- **Height:** 630px
- **Aspect Ratio:** 1200:630

## Fonts

- Inter
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | ✅ | 40 |
| `v_old` | text | ✅ | 15 |
| `v_new` | text | ✅ | 15 |
| `metric` | text | ✅ | 20 |
| `old_val` | text | ✅ | 15 |
| `new_val` | text | ✅ | 15 |
| `delta` | text | ✅ | 10 |
| `delta_dir` | text | ✅ | 4 |

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
cosy render --template version-compare --data input.json --output output.png
```
