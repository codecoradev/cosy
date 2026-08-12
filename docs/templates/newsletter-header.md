# Newsletter Header

Email newsletter header banner

![Newsletter Header sample](/samples/newsletter-header.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1200px |
| Height | 400px |
| Aspect | 1200:400 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `issue_number` | text | No | 10 | Newsletter issue number (e.g. "#12") |
| `title` | text | Yes | 120 | Main title or heading |
| `date` | text | No | 30 | Date string (e.g. "25 August 2026") |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "issue_number": "#12",
      "title": "Why Rust?",
      "date": "25 August 2026"
    }
  ]
}
```

## Render

```bash
cosy render --template newsletter-header --data input.json --output output.png
```
