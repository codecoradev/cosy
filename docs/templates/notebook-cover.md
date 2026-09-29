# Notebook Cover

Notebook/study-notes carousel cover: handwritten hero, tagline banner, 6-step pipeline with inline SVG icons, dashed intro box, and a 9-row table of contents with dot leaders. Pairs with `notebook-page` for the inner slides.

## Preview

![Notebook Cover](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/notebook-cover.png)

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
| `page_label` | text | yes | max 12 - Right header label (e.g. 'cover'). |
| `hero_title` | text | yes | max 18 - Big display title (Caveat). |
| `hero_font_size` | number | - | Hero font size (140-170). |
| `hero_subtitle` | text | yes | max 46 - Uppercase subtitle under hero. |
| `tagline` | text | yes | max 60 - Italic banner line. |
| `icon1` | text | - | Pipeline icon 1: note/phone/brain/search/spark/rocket. |
| `icon2` | text | - | Pipeline icon 2. |
| `icon3` | text | - | Pipeline icon 3. |
| `icon4` | text | - | Pipeline icon 4. |
| `icon5` | text | - | Pipeline icon 5. |
| `icon6` | text | - | Pipeline icon 6. |
| `label1` | text | yes | max 14 - Pipeline label 1. |
| `label2` | text | yes | max 14 - Pipeline label 2. |
| `label3` | text | yes | max 14 - Pipeline label 3. |
| `label4` | text | yes | max 14 - Pipeline label 4. |
| `label5` | text | yes | max 14 - Pipeline label 5. |
| `label6` | text | yes | max 14 - Pipeline label 6. |
| `intro_lead` | text | yes | max 20 - Bold lead inside dashed intro box (purple). |
| `intro_text` | text | yes | max 150 - Intro sentence (wrapped). |
| `toc_title` | text | yes | max 24 - Table-of-contents heading. |
| `toc1` | text | yes | max 22 |
| `pg1` | text | yes | max 8 |
| `toc2` | text | yes | max 22 |
| `pg2` | text | yes | max 8 |
| `toc3` | text | yes | max 22 |
| `pg3` | text | yes | max 8 |
| `toc4` | text | yes | max 22 |
| `pg4` | text | yes | max 8 |
| `toc5` | text | yes | max 22 |
| `pg5` | text | yes | max 8 |
| `toc6` | text | yes | max 22 |
| `pg6` | text | yes | max 8 |
| `toc7` | text | yes | max 22 |
| `pg7` | text | yes | max 8 |
| `toc8` | text | yes | max 22 |
| `pg8` | text | yes | max 8 |
| `toc9` | text | yes | max 22 |
| `pg9` | text | yes | max 8 |

## Usage

```bash
cosy render --template notebook-cover --data input.json --output output.png
```
