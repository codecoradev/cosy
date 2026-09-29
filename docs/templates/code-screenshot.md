# Code Screenshot

![Code Screenshot](https://s3.ajianaz.dev/hermes/codecoradev/cosy/preview/code-screenshot.png)

macOS-style editor window with **real syntax highlighting** for code snippets — the carbon.now.sh pattern.

## Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | text | no | Window title in the title bar (max 60) |
| `filename` | text | no | Filename shown next to the traffic lights (max 40) |
| `code_lang` | text | no | Language id: `rust`, `go`, `python`, `typescript`, `javascript`, `bash`, `sql` (max 12) |
| `theme` | text | no | `dark` (default) or `light` |
| `code` | text | yes | Source code; newlines separate lines (max 900). Lines beyond 13 are clipped by the code window |

## Syntax highlighting

The `code` field is automatically tokenized per line:

| Token class | Color |
|-------------|-------|
| Keywords (language-aware, word-bounded) | `#cba6f7` mauve |
| Strings (`"…"`, `'…'`, `` `…` ``) | `#a6e3a1` green |
| Numbers | `#fab387` peach |
| Comments (`//`, `#`, `--`) | `#6c7086` muted |

Unknown languages still get string/number/comment classes (language-independent); only keywords are language-specific.

## Example

```bash
cosy render --template code-screenshot --data data.json --output shot.png
```

```json
{
  "brand": { "brand_name": "ajianaz" },
  "slides": [
    {
      "title": "cosy/src/render.rs",
      "filename": "render.rs",
      "code_lang": "rust",
      "code": "pub fn render_slide_to_png() -> Result<Vec<u8>> {\n    let svg = process_template(template, data)?;\n    render_tree(&tree, scale)\n}"
    }
  ]
}
```
