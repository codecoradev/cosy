# Dev Quote

Code-style quote card, square

![Dev Quote sample](/samples/dev-quote.png)

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
| `code_snippet` | text | Yes | 500 | Code block content |
| `language` | text | No | 20 | Programming language label |
| `filename` | text | No | 30 | Filename for code display |

## Example JSON

```json
{
  "brand": {
    "brand_name": "Your Brand",
    "brand_handle": "@yourbrand"
  },
  "slides": [
    {
      "code_snippet": "fn main() { }",
      "language": "rust",
      "filename": "main.rs"
    }
  ]
}
```

## Render

```bash
cosy render --template dev-quote --data input.json --output output.png
```
