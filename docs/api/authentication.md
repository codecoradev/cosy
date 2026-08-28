# Authentication

Cosy uses bearer token authentication for the API.

## How It Works

1. Set `COSY_API_KEY` environment variable OR pass `--token` flag when starting the server
2. Clients send the token in the `Authorization` header
3. The token is compared using **constant-time comparison** to prevent timing attacks

## Configuration

### Environment Variable (Recommended)

```bash
export COSY_API_KEY=your-secure-token
cosy serve --port 3000
```

### CLI Flag

```bash
cosy serve --port 3000 --token your-secure-token
```

### Disabled (Development)

If neither is set, auth is **disabled**. The health endpoint reflects this:

```json
{
  "status": "ok",
  "version": "0.1.0",
  "templates": 18,
  "auth_enabled": false
}
```

## Usage

Clients must send the token as a Bearer token:

```bash
curl -X POST http://localhost:3000/api/render \
  -H "Authorization: Bearer your-secure-token" \
  -H "Content-Type: application/json" \
  -d '{"template":"stat-card","data":{...}}'
```

## Security

- **Constant-time comparison**: Token comparison uses XOR-accumulate to prevent timing side-channel attacks
- **Health endpoint**: Always public, no auth required — safe for load balancer health checks
- **No rate limiting**: Use a reverse proxy (Nginx, Cloudflare) for rate limiting and DDoS protection
