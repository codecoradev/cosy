# Newsletter Header

Email newsletter header banner

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1200px |
| Height | 400px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `issue_number` | text | No | 10 |  |
| `title` | text | Yes | 120 | Wraps at 35 chars |
| `date` | text | No | 30 |  |

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
cosy render --template newsletter-header --data input.json --output output.png
```
