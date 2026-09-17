# Notes App

## Preview

![Notes App](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/notes-app.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1350px
- **Aspect Ratio:** 1080:1350

## Fonts

- Inter

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | — | 40 |
| `body` | text | ✅ | 400 |
| `note_bg` | color | — | — |
| `title_color` | color | — | — |
| `body_color` | color | — | — |
| `note_timestamp` | text | — | 30 |

`body` supports inline markup: `*bold*`, `_italic_`, `*color:#hex*...*color*`.
`note_bg` defaults to the iOS notes yellow (`#fef9c3`).

## Brand Fields

| Field | Type | Required | Default |
|-------|------|----------|---------|
| `brand_name` | text | ✅ | — |
| `brand_handle` | text | ✅ | — |
| `bg_color` | bg | — | #1e1e2e |
| `bg_color_end` | bg | — | #11111b |
| `accent_color` | text | — | #cba6f7 |
| `accent_color_end` | text | — | #89b4fa |
| `bg_image` | image | — | — |
| `bg_image_opacity` | number | — | 0.15 |
| `bg_overlay_opacity` | number | — | 0.7 |
| `logo` | image | — | — |

## Usage

```bash
cosy render --template notes-app --data input.json --output output.png
```
