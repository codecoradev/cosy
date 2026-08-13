# Release Notes

Software release notes with version, date, and categorized changes

## Preview

![Release Notes](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/release-notes.png)

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
| `version` | text | ✅ | 20 |
| `date` | text | ✅ | 30 |
| `new1` | text | ✅ | 60 |
| `new2` | text | — | 60 |
| `new3` | text | — | 60 |
| `fix1` | text | ✅ | 60 |
| `fix2` | text | — | 60 |

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
cosy render --template release-notes --data input.json --output output.png
```
