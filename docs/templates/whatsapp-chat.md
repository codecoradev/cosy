# WhatsApp Chat

## Preview

![WhatsApp Chat](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/whatsapp-chat.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1350px
- **Aspect Ratio:** 1080:1350

## Fonts

- Inter

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `contact_name` | text | — | 24 |
| `contact_status` | text | — | 24 |
| `text_color` | color | — | — |
| `m1_text` | text | — | 120 |
| `m1_side` | text | — | 5 |
| `m2_text` | text | — | 120 |
| `m2_side` | text | — | 5 |
| `m3_text` | text | — | 120 |
| `m3_side` | text | — | 5 |
| `m4_text` | text | — | 120 |
| `m4_side` | text | — | 5 |
| `m5_text` | text | — | 120 |
| `m5_side` | text | — | 5 |
| `m6_text` | text | — | 120 |
| `m6_side` | text | — | 5 |

Message fields support inline markup: `*bold*`, `_italic_`, `*color:#hex*...*color*`.
Each `mN_side` is `left` (default) or `right`.

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
cosy render --template whatsapp-chat --data input.json --output output.png
```
