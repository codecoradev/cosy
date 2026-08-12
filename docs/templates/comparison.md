# Comparison

Before/after comparison

![Comparison sample](/samples/comparison.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1350px |
| Aspect | 1080:1350 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `title` | text | Yes | 80 | Main title or heading |
| `before_label` | text | No | 30 | Label for "before" section |
| `before_text` | text | No | 150 | Description of the before state |
| `after_label` | text | No | 30 | Label for "after" section |
| `after_text` | text | No | 150 | Description of the after state |

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
      "before_label": "Before",
      "before_text": "Manual design in Figma, every time",
      "after_label": "After",
      "after_text": "Automated rendering in 200ms"
    }
  ]
}
```

## Render

```bash
cosy render --template comparison --data input.json --output output.png
```
