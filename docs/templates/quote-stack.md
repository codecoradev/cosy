# Quote Stack

Multiple quotes stacked vertically, each with different author

## Preview

![Quote Stack](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/quote-stack.png)

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
| `title` | text | — | 60 |
| `q1_text` | text | ✅ | 120 |
| `q1_author` | text | ✅ | 40 |
| `q2_text` | text | ✅ | 120 |
| `q2_author` | text | ✅ | 40 |
| `q3_text` | text | ✅ | 120 |
| `q3_author` | text | ✅ | 40 |

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
cosy render --template quote-stack --data input.json --output output.png
```
