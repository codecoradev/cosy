# Cosy — Template Directory

Templates live here. Each template is a directory containing:

```
templates/
├── {template-name}/
│   ├── schema.json     # Template field definitions
│   └── template.svg    # SVG template with {{ minijinja tokens }}
```

## Creating a Template

1. Design your visual in Figma/Illustrator
2. Export as SVG
3. Replace dynamic content with `{{ field_name }}` tokens
4. Define field schema in `schema.json`
5. Test: `cosy render --template {name} --data test.json --output out.png`

## Schema Format

```json
{
  "id": "og-image",
  "name": "Open Graph Image",
  "dimensions": { "width": 1200, "height": 630 },
  "fonts": ["Inter"],
  "brand_fields": {
    "logo": { "type": "image", "required": true },
    "brand_name": { "type": "text", "required": true, "max": 30 },
    "url": { "type": "text", "required": false }
  },
  "slide_fields": {
    "background": { "type": "bg", "required": false, "default": "#1a1b26" },
    "title": { "type": "text", "required": true, "max": 80 },
    "subtitle": { "type": "text", "required": false, "max": 150 }
  }
}
```
