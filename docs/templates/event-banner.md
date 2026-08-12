# Event Banner

Event/webinar banner

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1920px |
| Height | 1080px |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `event_name` | text | Yes | 100 | Wraps at 30 chars |
| `date` | text | No | 30 |  |
| `time` | text | No | 20 |  |
| `speaker` | text | No | 50 |  |

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
      "event_name": "Sample event name"
    }
  ]
}
```

```bash
cosy render --template event-banner --data input.json --output output.png
```
