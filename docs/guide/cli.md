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
| `-o, --output` | path | Yes | Output image path (used verbatim — match the extension to `--format`) |
| `-s, --scale` | float | No | Scale factor (default: 2.0 — retina/2x output) |
| `--format` | enum | No | Output container: `png` (default), `webp`, or `svg` (vector, text-to-path — `--scale` ignored) |
| `--metadata` | string | No | JSON string echoed in the `--json-output` result (pipeline tracing) |
| `--dataset` | path | No | CSV file: one render per row (see below) |
| `--fail-fast` | flag | No | With `--dataset`: stop unscheduled rows after the first failure |

**CSV batch rendering:**

```bash
cosy render --template certificate --dataset attendees.csv --out-dir certs/
```

- CSV header row = field names; each data row renders one image.
- `number`/`boolean` schema fields are parsed from their string values
  automatically; brand fields default to the template's `defaults.json`.
- Optional `_filename` column overrides the output name (sanitized);
  otherwise files are `NNNN_<slug>.<ext>`.
- Optional `_data` column carries a full JSON slide object or array
  (multi-slide rows render to `{stem}_slides/NN.<ext>`). Quote cells
  containing commas.
- Exit code is 1 when any row fails; per-row errors are reported at the
  end. `--json-output` emits `{template, total_rows, succeeded, failed,
  files, errors, render_time_ms}`.

**Example:**

```bash
cosy render --template og-image --data post.json --output cover.png --scale 2.0
```

**WebP output:**

```bash
cosy render --template social-quote --data quote.json --output quote.webp --format webp
```

::: tip
WebP encoding is lossless (via the `image` crate — no extra dependencies).
Multi-slide renders into a directory write `NN.webp` files when
`--format webp` is set. Single-file `-o` paths are used exactly as given.
:::

::: tip
`render` validates input against the template schema **before** rendering. Invalid input
(missing required fields, over-max text, wrong field type) fails fast with field-level
messages — e.g. `field 'b1_value' must be a number, got string`. With `--json-output`,
validation errors are returned as machine-readable JSON.
:::

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
