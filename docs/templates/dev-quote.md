# Dev Quote

Code-style quote card, square

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1080px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `code_snippet` | text | Yes | 500 | Wraps at 45 chars |
| `language` | text | No | 20 |  |
| `filename` | text | No | 30 |  |

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
      "code_snippet": "Sample code snippet"
    }
  ]
}
```

```bash
cosy render --template dev-quote --data input.json --output output.png
```
