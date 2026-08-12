# Checklist

Tips/checklist carousel slide

![Checklist sample](/samples/checklist.png)

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
| `title` | text | Yes | 100 | Main title or heading |
| `item1` | text | No | 80 | Checklist item 1 |
| `item2` | text | No | 80 | Checklist item 2 |
| `item3` | text | No | 80 | Checklist item 3 |
| `item4` | text | No | 80 | Checklist item 4 |
| `item5` | text | No | 80 | Checklist item 5 |

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
      "item1": "Write tests",
      "item2": "Run clippy",
      "item3": "Update changelog",
      "item4": "Tag release",
      "item5": "Publish"
    }
  ]
}
```

## Render

```bash
cosy render --template checklist --data input.json --output output.png
```
