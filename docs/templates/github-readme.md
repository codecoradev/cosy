# GitHub README Banner

README banner for repos

![GitHub README Banner sample](/samples/github-readme.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1280px |
| Height | 640px |
| Aspect | 1280:640 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `title` | text | Yes | 100 | Main title or heading |
| `description` | text | No | 300 | Brief description |
| `tech_stack` | text | No | 100 | Comma-separated tech list |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "title": "Why Rust?",
      "description": "Template-based image generation in Rust.",
      "tech_stack": "Rust, resvg, axum"
    }
  ]
}
```

## Render

```bash
cosy render --template github-readme --data input.json --output output.png
```
