# Dockerfile

Dockerfile instructions viewer

## Preview

![Dockerfile](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/dockerfile.png)

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
| `title` | text | ✅ | 30 |
| `step1_inst` | text | ✅ | 8 |
| `step1_arg` | text | ✅ | 50 |
| `step2_inst` | text | ✅ | 8 |
| `step2_arg` | text | ✅ | 50 |
| `step3_inst` | text | ✅ | 8 |
| `step3_arg` | text | ✅ | 50 |
| `step4_inst` | text | — | 8 |
| `step4_arg` | text | — | 50 |
| `step5_inst` | text | — | 8 |
| `step5_arg` | text | — | 50 |

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
cosy render --template dockerfile --data input.json --output output.png
```
