# Podcast Cover

Square podcast/album cover

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1080px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `episode_num` | text | No | 20 |  |
| `title` | text | Yes | 150 | Wraps at 22 chars |
| `guest` | text | No | 50 |  |

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
cosy render --template podcast-cover --data input.json --output output.png
```
