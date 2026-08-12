# CLI Reference

## Synopsis

```
cosy [COMMAND] [OPTIONS]
```

## Commands

### `render`

Render an image from a template.

```bash
cosy render --template <TEMPLATE> --data <DATA.json> --output <OUTPUT.png>
```

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `-t, --template` | string | Yes | Template name (e.g. `stat-card`) |
| `-d, --data` | path | Yes | Path to JSON input file |
| `-o, --output` | path | Yes | Output PNG path |
| `-s, --scale` | float | No | Scale factor (default: 1.0) |

**Example:**

```bash
cosy render --template og-image --data post.json --output cover.png --scale 2.0
```

### `serve`

Start the HTTP API server.

```bash
cosy serve --port <PORT> [--token <TOKEN>]
```

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `-p, --port` | u16 | Yes | Port to listen on |
| `--token` | string | No | Bearer token for auth. If empty, `COSY_API_KEY` env var is used. |

::: tip
If neither `--token` nor `COSY_API_KEY` is set, auth is disabled (development mode). The health endpoint is always public.
:::

### `templates`

List all available templates.

```bash
cosy templates
```

### `validate`

Validate JSON input against a template's schema.

```bash
cosy validate --template <TEMPLATE> --data <DATA.json>
```

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `-t, --template` | string | Yes | Template name |
| `-d, --data` | path | Yes | Path to JSON input file |

### `--version`

```bash
cosy --version
# cosy 0.1.0
```

### `--help`

```bash
cosy --help
cosy render --help
cosy serve --help
```
