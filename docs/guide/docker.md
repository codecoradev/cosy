# Docker

## Pull

```bash
docker pull ghcr.io/codecoradev/cosy:latest
# Or specific version
docker pull ghcr.io/codecoradev/cosy:v0.1.0
```

::: warning
The GHCR image is **private**. Authenticate first:

```bash
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin
```
:::

## Run

### CLI Mode

```bash
docker run --rm \
  -v $(pwd)/data:/data \
  ghcr.io/codecoradev/cosy:latest \
  cosy render --template stat-card --data /data/input.json --output /data/output.png
```

### Server Mode

```bash
docker run -d \
  --name cosy \
  -p 3000:3000 \
  -e COSY_API_KEY=your-secret \
  ghcr.io/codecoradev/cosy:latest \
  cosy serve --port 3000
```

## Docker Compose

```yaml
services:
  cosy:
    image: ghcr.io/codecoradev/cosy:latest
    ports:
      - "3000:3000"
    environment:
      - COSY_API_KEY=your-secret
    command: cosy serve --port 3000
    restart: unless-stopped
```

```bash
docker compose up -d
```

## Verify

```bash
curl http://localhost:3000/api/health
# {"status":"ok","version":"0.1.0","templates":18,"auth_enabled":true}
```

## Image Details

| Property | Value |
|----------|-------|
| Base image | `debian:bookworm-slim` |
| Architecture | amd64, arm64 |
| Size | ~50MB |
| Dependencies | `libfontconfig1` only |
