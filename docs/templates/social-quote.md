# Social Quote



## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1350px |
| Fonts | Inter |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `quote` | text | Yes | 280 | Wraps at 35 chars |
| `author` | text | Yes | 50 |  |
| `author_title` | text | No | 50 |  |

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
      "quote": "Sample quote",
      "author": "Sample author"
    }
  ]
}
```

```bash
cosy render --template social-quote --data input.json --output output.png
```
