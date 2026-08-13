# Code Snippet

Code snippet card for sharing code on social media

## Preview

![Code Snippet](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/code-snippet.png)

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
| `title` | text | — | 80 |
| `language` | text | — | 20 |
| `line1` | text | ✅ | 70 |
| `line2` | text | — | 70 |
| `line3` | text | — | 70 |
| `line4` | text | — | 70 |
| `line5` | text | — | 70 |
| `line6` | text | — | 70 |
| `line7` | text | — | 70 |
| `line8` | text | — | 70 |

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
cosy render --template code-snippet --data input.json --output output.png
```
