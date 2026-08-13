# Terminal Output

Terminal window showing command output with colored lines

## Preview

![Terminal Output](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/terminal-output.png)

## Dimensions

- **Width:** 1200px
- **Height:** 630px
- **Aspect Ratio:** 1200:630

## Fonts

- JetBrains Mono
- Inter
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | ✅ | 60 |
| `cmd` | text | ✅ | 60 |
| `line1` | text | ✅ | 60 |
| `line2` | text | — | 60 |
| `line3` | text | — | 60 |
| `line4` | text | — | 60 |

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
cosy render --template terminal-output --data input.json --output output.png
```
