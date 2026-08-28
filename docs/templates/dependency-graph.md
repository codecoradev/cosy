# Dependency Graph

Dependency tree visualization

## Preview

![Dependency Graph](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/dependency-graph.png)

## Dimensions

- **Width:** 1080px
- **Height:** 1080px
- **Aspect Ratio:** 1080:1080

## Fonts

- Inter
- Space Grotesk

## Slide Fields

| Field | Type | Required | Max Length |
|-------|------|----------|------------|
| `title` | text | ✅ | 40 |
| `root` | text | ✅ | 20 |
| `child1` | text | ✅ | 20 |
| `child2` | text | ✅ | 20 |
| `child3` | text | — | 20 |
| `leaf1` | text | — | 20 |
| `leaf2` | text | — | 20 |
| `leaf3` | text | — | 20 |
| `leaf4` | text | — | 20 |

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
cosy render --template dependency-graph --data input.json --output output.png
```
