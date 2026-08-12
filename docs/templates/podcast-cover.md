# Podcast Cover

Square podcast/album cover

![Podcast Cover sample](/samples/podcast-cover.png)

## Dimensions

| Property | Value |
|----------|-------|
| Width | 1080px |
| Height | 1080px |
| Aspect | 1080:1080 |
| Fonts | Inter, JetBrains Mono |

## Slide Fields

| Field | Type | Required | Max | Description |
|-------|------|----------|-----|-------------|
| `episode_num` | text | No | 20 | Episode number label (e.g. "EP 03") |
| `title` | text | Yes | 150 | Main title or heading |
| `guest` | text | No | 50 | Guest name for podcasts |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "episode_num": "EP 03",
      "title": "Why Rust?",
      "guest": "w/ Ajian"
    }
  ]
}
```

## Render

```bash
cosy render --template podcast-cover --data input.json --output output.png
```
