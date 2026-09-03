# HTTP Server

## Starting the Server

```bash
# Development (no auth)
cosy serve --port 3000

# Production (with auth)
COSY_API_KEY=your-secret cosy serve --port 3000

# Or via flag
cosy serve --port 3000 --token your-secret
```

## Remote Image Security

By default, image URLs (`bg_image`, `logo`) pointing at **private/internal
addresses** (loopback, RFC1918, link-local — including cloud metadata
`169.254.169.254`) are blocked, with the same check applied on every redirect
hop. This prevents SSRF through attacker-controlled render data. Blocked
fetches log a warning server-side and the render proceeds without the image.

To allow internal image sources (e.g. an internal MinIO), opt in explicitly:

```bash
cosy serve --port 3000 --allow-private-images
```

## Health Check

Always public, no auth required.

```bash
curl http://localhost:3000/api/health
```

```json
{
  "status": "ok",
  "version": "0.1.0",
  "templates": 18,
  "auth_enabled": true
}
```

## Render Endpoint

### `POST /api/render`

Render an image from a template.

**Headers:**

| Header | Required | Description |
|--------|----------|-------------|
| `Content-Type: application/json` | Yes | |
| `Authorization: Bearer <token>` | If auth enabled | Bearer token |

**Body:**

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
        "source": "Cosy v0.1.0 CI"
      }
    ]
  },
  "scale": 1.0
}
```

**Response:** PNG image (`Content-Type: image/png`)

### Example

```bash
curl -X POST http://localhost:3000/api/render \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-secret" \
  -d '{
    "template": "quote-card",
    "data": {
      "brand": { "brand_name": "CodeCora" },
      "slides": [{
        "quote": "Ship boring, iterate fast.",
        "author": "Senior Dev"
      }]
    }
  }' \
  -o quote.png
```

## Error Responses

### 401 Unauthorized

```json
{
  "error": "unauthorized",
  "message": "Missing or invalid Authorization header"
}
```

### 400 Bad Request

```json
{
  "error": "invalid_request",
  "message": "Template 'unknown' not found"
}
```

## CORS

CORS is enabled by default with `CorsLayer::permissive()`. For production, place behind a reverse proxy with restricted CORS headers.
