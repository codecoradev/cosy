# Endpoints

## GET /api/health

Returns server health status. **Always public** — no auth required.

### Response

```json
{
  "status": "ok",
  "version": "0.1.0",
  "templates": 18,
  "auth_enabled": true
}
```

| Field | Type | Description |
|-------|------|-------------|
| `status` | string | Always `"ok"` if server is running |
| `version` | string | Cosy version |
| `templates` | number | Number of bundled templates |
| `auth_enabled` | boolean | Whether bearer token auth is active |

---

## POST /api/render

Renders an image from a template.

### Request

**Headers:**

| Header | Value | Required |
|--------|-------|----------|
| `Content-Type` | `application/json` | Yes |
| `Authorization` | `Bearer <token>` | If auth enabled |

**Body:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `template` | string | Yes | Template name (e.g. `"stat-card"`) |
| `data` | object | Yes | Template input data (brand + slides) |
| `scale` | float | No | Scale factor (default: `1.0`) |

**Example:**

```json
{
  "template": "stat-card",
  "data": {
    "brand": {
      "brand_name": "CodeCora",
      "brand_handle": "@codecoradev",
      "bg_color": "#1e1e2e",
      "bg_color_end": "#11111b",
      "accent_color": "#cba6f7",
      "accent_color_end": "#89b4fa"
    },
    "slides": [
      {
        "stat_number": "123",
        "stat_label": "Tests Passing",
        "source": "CI"
      }
    ]
  },
  "scale": 1.0
}
```

### Responses

#### 200 OK

Returns the rendered PNG image.

| Header | Value |
|--------|-------|
| `Content-Type` | `image/png` |

Body: PNG binary data.

#### 400 Bad Request

Invalid template name, missing required fields, or malformed JSON.

```json
{
  "error": "invalid_request",
  "message": "Template 'unknown' not found"
}
```

Common causes:

| Message | Fix |
|---------|-----|
| `Template 'X' not found` | Check `cosy templates` for valid names |
| `missing field 'brand'` | Add `brand` object to your data |
| `missing field 'slides'` | Add `slides` array to your data |

#### 401 Unauthorized

Missing or invalid Authorization header.

```json
{
  "error": "unauthorized",
  "message": "Missing or invalid Authorization header"
}
```

#### 500 Internal Server Error

Rendering failed unexpectedly.

```json
{
  "error": "render_failed",
  "message": "Failed to process image data"
}
```
