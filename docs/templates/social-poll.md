# Social Poll

Poll card with question and visual percentage bars

## Preview

![Social Poll](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/social-poll.png)

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
| `question` | text | ✅ | 100 |
| `opt1` | text | ✅ | 40 |
| `opt1_pct` | text | ✅ | 10 |
| `opt2` | text | ✅ | 40 |
| `opt2_pct` | text | ✅ | 10 |
| `opt3` | text | — | 40 |
| `opt3_pct` | text | — | 10 |

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
cosy render --template social-poll --data input.json --output output.png
```
