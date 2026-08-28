# API Reference

Cosy provides a simple HTTP API for rendering images from templates.

## Base URL

```
http://localhost:3000/api
```

## Content Type

All request bodies must be `application/json`. All image responses are `image/png`.

## Endpoints

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| `GET` | `/api/health` | No | Health check |
| `POST` | `/api/render` | Yes* | Render an image |

*\*Auth required only when `COSY_API_KEY` is set.*

## Quick Reference

### Health Check

```bash
curl http://localhost:3000/api/health
```

### Render Image

```bash
curl -X POST http://localhost:3000/api/render \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <token>" \
  -d '{"template":"stat-card","data":{...}}' \
  -o output.png
```

See [Endpoints](./endpoints) for detailed request/response schemas.
