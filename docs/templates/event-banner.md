# Event Banner

Event/webinar banner

![Event Banner sample](/samples/event-banner.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1920px |
| Height | 1080px |
| Aspect | 1920:1080 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `event_name` | text | Yes | 100 | Full event name |
| `date` | text | No | 30 | Date string (e.g. "25 August 2026") |
| `time` | text | No | 20 | Time string (e.g. "19:00 WIB") |
| `speaker` | text | No | 50 | Speaker name for events |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "event_name": "Rust Meetup Indonesia",
      "date": "25 August 2026",
      "time": "19:00 WIB",
      "speaker": "Ajian Azhari"
    }
  ]
}
```

## Render

```bash
cosy render --template event-banner --data input.json --output output.png
```
