# Rust Snippet

Rust code snippet card with syntax highlighting style

## Preview

![Rust Snippet](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/rust-snippet.png)

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
| `title` | text | ✅ | 60 |
| `code_line1` | text | ✅ | 60 |
| `code_line2` | text | — | 60 |
| `code_line3` | text | — | 60 |
| `code_line4` | text | — | 60 |
| `code_line5` | text | — | 60 |

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
cosy render --template rust-snippet --data input.json --output output.png
```
