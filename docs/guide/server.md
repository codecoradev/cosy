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

Remote image URLs (`bg_image`, `logo`) are fetched with an SSRF guard:
- only `https://` URLs (plain `http://` needs the opt-in below)
- the target must resolve to a globally routable address — loopback, RFC1918,
  and link-local (incl. cloud metadata `169.254.169.254`) are rejected, and the
  connection is pinned to the validated IP so DNS rebinding cannot bypass it
- redirects are not followed; bodies are streamed with a hard 10 MB cap
- blocked fetches log a warning server-side and the render proceeds without
  the image (no error details are returned to the caller)

Local filesystem paths in `bg_image` / `logo` are **rejected by default**
too — a render request cannot make the server read its own files. Both
restrictions have explicit opt-ins:

```bash
# allow private/internal network targets (e.g. an internal MinIO)
cosy serve --port 3000 --allow-private-images

# allow local filesystem paths (the standalone CLI always allows these)
cosy serve --port 3000 --allow-local-image-paths
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
