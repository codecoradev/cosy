# Poll Result

Poll results visualization with percentage bars

## Preview

![Poll Result](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/poll-result.png)

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
| `question` | text | ✅ | 120 |
| `option_a` | text | ✅ | 60 |
| `option_a_pct` | text | ✅ | 10 |
| `option_b` | text | ✅ | 60 |
| `option_b_pct` | text | ✅ | 10 |
| `option_c` | text | — | 60 |
| `option_c_pct` | text | — | 10 |
| `option_d` | text | — | 60 |
| `option_d_pct` | text | — | 10 |
| `total_votes` | text | — | 30 |

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
cosy render --template poll-result --data input.json --output output.png
```
