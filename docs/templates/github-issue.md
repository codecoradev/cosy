# GitHub Issue

GitHub-style issue card with labels, title, and status

## Preview

![GitHub Issue](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/github-issue.png)

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
| `issue_num` | text | ✅ | 10 |
| `title` | text | ✅ | 80 |
| `status` | text | — | 20 |
| `label1` | text | — | 20 |
| `label2` | text | — | 20 |
| `author` | text | — | 30 |
| `comments` | text | — | 10 |

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
cosy render --template github-issue --data input.json --output output.png
```
