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
| `slide_index` | int | No | Zero-based slide to render with the default `png` format (default: `0`) |
| `response_format` | string | No | `"png"` (default, binary image) or `"json"` (all slides as base64 entries) |
| `image_format` | string | No | `"png"` (default) or `"webp"` — container for the rendered bytes, independent of `response_format` |
| `metadata` | any | No | Arbitrary JSON echoed back in the JSON envelope (pipeline tracing). Max 4 KB serialized. Binary responses log it instead |

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

**Multi-slide example** — render every slide of a carousel as base64 PNGs:

```json
{
  "template": "carousel-default",
  "response_format": "json",
  "scale": 0.5,
  "data": {
    "brand": {"brand_name": "CodeCora"},
    "slides": [
      {"eyebrow": "s1", "headline": "Slide One", "body": "first"},
      {"eyebrow": "s2", "headline": "Slide Two", "body": "second"}
    ]
  }
}
```

The JSON response contains per-slide base64 images. The field name stays
`png_base64` for backward compatibility; the actual container is reported in
`image_format` (defaults to `"png"` when `image_format` is not requested):

```json
{
  "template": "carousel-default",
  "slides": 2,
  "width": 540,
  "height": 675,
  "data": [
    {"index": 0, "png_base64": "iVBORw0KGgo...", "image_format": "png"},
    {"index": 1, "png_base64": "iVBORw0KGgo...", "image_format": "png"}
  ]
}
```

**WebP example** — request `image_format: "webp"` to receive lossless WebP
bytes instead of PNG. Works with both response formats:

```json
{
  "template": "stat-card",
  "image_format": "webp",
  "data": {
    "brand": {"brand_name": "CodeCora"},
    "slides": [{"stat_number": "123", "stat_label": "Tests Passing", "source": "CI"}]
  }
}
```

The binary response then has `Content-Type: image/webp`; a JSON envelope's
per-slide `image_format` becomes `"webp"`.

**Metadata example** — tag a render with a correlation ID and get it back:

```json
{
  "template": "carousel-default",
  "response_format": "json",
  "metadata": {"job_id": "render-42", "source": "blog-engine"},
  "data": { "brand": {"brand_name": "CodeCora"}, "slides": [ ... ] }
}
```

The JSON envelope echoes `metadata` verbatim. With the default binary
response the metadata is recorded in the server log instead (the body is
Metadata larger than 4 KB serialized is rejected with `413`.

### GET /r/{template}.{ext} — signed render URLs

Dynamic-image endpoint for OG tags: embed a plain URL, no client library.

```
GET /r/og-image.png?d=<base64url(JSON)>&sig=<hex hmac-sha256>
```

- `d` = base64url-encoded input JSON (`{"data": {...}}` or a bare
  brand+slides object); max 8 KB decoded → `413`.
- `sig` = HMAC-SHA256 hex over `{template}:{d}` using the API key as the
  signing key. Wrong/missing signature → `403`. Signatures are bound to
  the template name, so a URL for one template can't render another.
- `{ext}` selects the container: `.png` (default) or `.webp`.
- Responses carry `Cache-Control: public, max-age=3600` for re-crawls.
- Disabled with `404` when the server has no API key configured
  (dev mode keeps unsigned POST only).

Generating a URL (pseudo-code): `d = base64url(json); sig =
hmac_sha256_hex(api_key, template + ":" + d)`.

### Responses

#### 200 OK

`response_format: "png"` (default) returns the rendered image. With a
`slide_index`, that specific slide is rendered; without one, the first slide.
The `Content-Type` follows `image_format`: `image/png` (default) or
`image/webp`.

| Header | Value |
|--------|-------|
| `Content-Type` | `image/png` or `image/webp` |

Body: image binary data.

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
