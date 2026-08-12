# Testimonial

Customer testimonial card

![Testimonial sample](/samples/testimonial.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1200px |
| Aspect | 1080:1200 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `quote` | text | Yes | 300 | The quote text |
| `author_name` | text | Yes | 50 | Name of the person giving the testimonial |
| `author_role` | text | No | 50 | Role of the person (e.g. "Senior Engineer") |
| `author_company` | text | No | 50 | Company name |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "quote": "The best time to ship was yesterday. The second best is now.",
      "author_name": "Sarah Chen",
      "author_role": "Content Lead",
      "author_company": "DevStudio"
    }
  ]
}
```

## Render

```bash
cosy render --template testimonial --data input.json --output output.png
```
