# Github Readme

README banner for repos

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1280px |
| Height | 640px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `title` | text | Yes | 100 | Wraps at 35 chars |
| `description` | text | No | 300 | Wraps at 60 chars |
| `tech_stack` | text | No | 100 |  |

## Example

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "bg_color": "#1e1e2e",
    "bg_color_end": "#11111b",
    "accent_color": "#cba6f7",
    "accent_color_end": "#89b4fa"
  },
  "slides": [
    {
      "title": "Sample title"
    }
  ]
}
```

```bash
cosy render --template github-readme --data input.json --output output.png
```
