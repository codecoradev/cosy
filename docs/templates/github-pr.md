# GitHub PR

GitHub-style pull request card with merge status

## Preview

![GitHub PR](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/github-pr.png)

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
| `pr_num` | text | ✅ | 10 |
| `title` | text | ✅ | 80 |
| `branch` | text | ✅ | 40 |
| `status` | text | — | 20 |
| `additions` | text | — | 10 |
| `deletions` | text | — | 10 |
| `files_changed` | text | — | 10 |

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
cosy render --template github-pr --data input.json --output output.png
```
