# Notebook Page

Notebook/study-notes content page: chapter title box, star bullets, dashed example box (optional), cloud punchline callout, optional second dashed box, and a bottom brand line. Pairs with `notebook-cover`.

## Preview

![Notebook Page](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/notebook-page.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1350px
- **Aspect Ratio:** 1080:1350

## Fonts

- Kalam
- Caveat
- Inter

## Brand Fields

| Field | Type | Required | Max / Default |
|---|---|---|---|
| `accent_color` | text | - | #1e3a8a |
| `accent_color_end` | text | - | #6d28d9 |
| `bg_color` | text | - | #fdfcf5 |
| `bg_color_end` | text | - | #fdfcf5 |
| `bg_image` | image | - | - |
| `bg_image_opacity` | number | - | 0.15 |
| `logo` | image | - | - |
| `show_brand` | boolean | - | false |
| `brand_handle` | text | - | @ajianaz |
| `brand_name` | text | yes | ajianaz |

## Slide Fields

| Field | Type | Required | Max / Default |
|---|---|---|---|
| `header_handle` | text | yes | max 30 - Left header text (e.g. IG handle). |
| `page_label` | text | yes | max 12 - Right header label (e.g. 'Page 1'). |
| `chapter_title` | text | yes | max 30 - Chapter title inside the box. |
| `lead_text` | text | yes | max 140 - Lead sentence (wrapped). |
| `bullet1` | text | - | max 120 - Bullet point 1. |
| `bullet2` | text | - | max 120 - Bullet point 2. |
| `bullet3` | text | - | max 120 - Bullet point 3. |
| `box_title` | text | - | max 30 - Dashed box title (omit to hide box). |
| `box_text` | text | - | max 200 - Dashed box body (wrapped). |
| `box_color` | color | - | Dashed box border/title color (default accent). |
| `punchline` | text | yes | max 90 - Cloud callout punchline (wrapped). |
| `extra_title` | text | - | max 30 - Second dashed box title (omit to hide). |
| `extra_text` | text | - | max 180 - Second dashed box body (wrapped). |
| `footer_line` | text | - | max 60 - Bottom center line (brand/CTA). |

## Usage

```bash
cosy render --template notebook-page --data input.json --output output.png
```
