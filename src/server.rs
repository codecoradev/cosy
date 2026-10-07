//! HTTP API server for Cosy.
//!
//! Endpoints:
//! - GET  /api/health     — health check (public, no auth)
//! - GET  /api/templates  — list all templates (auth required)
//! - POST /api/render     — render template with JSON data → PNG (auth required)
//!
//! Authentication: Bearer token via `Authorization: Bearer <token>` header.
//! Set COSY_API_KEY env var or pass --token flag. If not set, auth is disabled (dev mode).

use crate::render;
use crate::schema::InputData;
use crate::template;
use axum::{
    extract::{Path, Query, Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

/// Shared server state — font DB built once at startup.
struct AppState {
    font_db: Arc<usvg::fontdb::Database>,
    /// Template count captured at startup (health checks must stay cheap).
    template_count: usize,
    /// Bounds concurrent renders so a burst of requests cannot exhaust
    /// CPU/memory (each render allocates a full-size pixmap).
    render_slots: tokio::sync::Semaphore,
    image_policy: crate::text::ImagePolicy,
    api_key: Option<String>,
    /// Signing key for GET /r/:template signed render URLs. Falls back to
    /// the API key when set; route stays disabled when neither exists.
    signing_key: Option<String>,
}

/// Request body for POST /api/render.
#[derive(Debug, Deserialize)]
pub struct RenderRequest {
    /// Template name (e.g. "stat-card") or path to template directory.
    pub template: String,
    /// Input data: brand fields + slides.
    pub data: InputData,
    /// Scale factor (default 2.0).
    #[serde(default = "default_scale")]
    pub scale: f32,
    /// Zero-based slide to render when responding with `image/png`.
    /// Defaults to the first slide. Ignored by the `json` response format,
    /// which renders every slide.
    #[serde(default)]
    pub slide_index: Option<usize>,
    /// Response format: `png` (default, binary image) or `json` (rendered
    /// slides as base64 PNG entries with metadata).
    #[serde(default)]
    pub response_format: Option<ResponseFormat>,
    /// Image container format for the rendered bytes: `png` (default) or
    /// `webp` (lossless). Orthogonal to `response_format`: a JSON
    /// envelope can carry PNG or WebP entries.
    #[serde(default)]
    pub image_format: Option<ImageFormatArg>,
    /// Arbitrary caller metadata echoed back in the JSON envelope
    /// (pipeline tracing / correlation IDs). Any JSON value, capped at
    /// [`MAX_METADATA_BYTES`] when serialized. Binary responses cannot
    /// carry it (the body is raw image bytes) — it is logged instead.
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Maximum serialized size of the optional `metadata` field.
pub const MAX_METADATA_BYTES: usize = 4096;

/// Accepted `scale` range for POST /api/render. The pixmap is
/// `width*scale × height*scale` RGBA and is copied twice while encoding, so an
/// unbounded value lets one request exhaust memory (and `panic = "abort"`
/// takes the whole server down with it).
pub const MIN_SCALE: f32 = 0.1;
pub const MAX_SCALE: f32 = 4.0;

/// Maximum slides rendered by one request (`response_format: "json"`).
pub const MAX_SLIDES_PER_REQUEST: usize = 20;

/// True for a plain template id (`stat-card`). The HTTP API only serves
/// templates from `./templates`; path-like values (`/tmp/x`, `../x`, `a/b`)
/// would let callers render arbitrary directories on the server.
fn is_safe_template_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

/// Image container format for POST /api/render (`image_format` field).
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormatArg {
    Png,
    Webp,
    Svg,
}

impl From<ImageFormatArg> for crate::format::OutputFormat {
    fn from(arg: ImageFormatArg) -> Self {
        match arg {
            ImageFormatArg::Png => Self::Png,
            ImageFormatArg::Webp => Self::WebP,
            ImageFormatArg::Svg => Self::Svg,
        }
    }
}

/// Response format for POST /api/render.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseFormat {
    /// Raw PNG bytes (`image/png`).
    Png,
    /// JSON envelope with per-slide base64 PNGs (`application/json`).
    Json,
}

fn default_scale() -> f32 {
    2.0
}

/// One rendered slide in a JSON response.
#[derive(Debug, Serialize)]
pub struct RenderedSlide {
    /// Zero-based slide index.
    pub index: usize,
    /// Base64-encoded image bytes. Field name kept as `png_base64` for
    /// backward compatibility — the actual container is `image_format`.
    pub png_base64: String,
    /// Container format of the encoded bytes ("png" or "webp").
    pub image_format: &'static str,
}

/// JSON response envelope for `response_format: "json"`.
#[derive(Debug, Serialize)]
pub struct RenderResponse {
    pub template: String,
    /// Rendered slide count.
    pub slides: usize,
    /// Dimensions of the template canvas (scale applied).
    pub width: u32,
    pub height: u32,
    pub data: Vec<RenderedSlide>,
    /// Caller metadata echoed verbatim (absent when not provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Response for GET /api/health.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub templates: usize,
    pub auth_enabled: bool,
}

/// Response wrapper for errors.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// ─── GET signed render URLs (#95) ────────────────────────────────────

/// Maximum serialized size of the `d` query payload.
pub const MAX_SIGNED_PAYLOAD_BYTES: usize = 8 * 1024;

/// HMAC-SHA256 hex signature over a payload using the signing key.
fn sign_payload(key: &str, payload: &[u8]) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let mut mac =
        Hmac::<Sha256>::new_from_slice(key.as_bytes()).expect("HMAC accepts any key length");
    mac.update(payload);
    let bytes = mac.finalize().into_bytes();
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Constant-time string comparison for signature checks.
fn constant_time_str_eq(a: &str, b: &str) -> bool {
    constant_time_eq(a.as_bytes(), b.as_bytes())
}

/// Parse and verify a signed GET render request.
/// Returns the decoded input data, or an HTTP status + message.
fn verify_signed_request(
    signing_key: &str,
    template_id: &str,
    query_d: &str,
    query_sig: &str,
) -> Result<InputData, (StatusCode, String)> {
    // Payload = template + data so URLs can't be transplanted across
    // templates (a signature for template A must not render template B).
    let payload = format!("{template_id}:{query_d}");

    if !constant_time_str_eq(&sign_payload(signing_key, payload.as_bytes()), query_sig) {
        return Err((StatusCode::FORBIDDEN, "invalid signature".into()));
    }

    // d = base64url(JSON) — decode leniently (padded or not).
    use base64::Engine;
    let json_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(query_d)
        .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(query_d))
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "d is not valid base64url-encoded JSON".into(),
            )
        })?;

    if json_bytes.len() > MAX_SIGNED_PAYLOAD_BYTES {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            format!(
                "payload too large: {} bytes (max {})",
                json_bytes.len(),
                MAX_SIGNED_PAYLOAD_BYTES
            ),
        ));
    }

    let value: serde_json::Value = serde_json::from_slice(&json_bytes).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("d does not contain valid JSON: {e}"),
        )
    })?;

    // Accept {"data": {...}} or a bare data object (brand+slides).
    match value.get("data") {
        Some(d) => serde_json::from_value(d.clone()).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("data does not match input schema: {e}"),
            )
        }),
        None => serde_json::from_value(value).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("d does not match input schema: {e}"),
            )
        }),
    }
}

/// Build a signed GET render URL (used by tests; documented for callers
/// generating `<meta property="og:image">` tags).
pub fn signed_get_url(
    base: &str,
    signing_key: &str,
    template: &str,
    data_json: &str,
    ext: &str,
) -> String {
    use base64::Engine;
    let d = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data_json.as_bytes());
    let payload = format!("{template}:{d}");
    let sig = sign_payload(signing_key, payload.as_bytes());
    format!("{base}/r/{template}.{ext}?d={d}&sig={sig}")
}

/// Treat an empty string as "not set".
fn non_blank(value: Option<String>) -> Option<String> {
    value.filter(|v| !v.is_empty())
}

/// Start the HTTP server.
///
/// If `api_key` is Some, all endpoints except /api/health require
/// `Authorization: Bearer <api_key>` header.
pub async fn run(
    port: u16,
    api_key: Option<String>,
    image_policy: crate::text::ImagePolicy,
) -> anyhow::Result<()> {
    run_on("0.0.0.0", port, api_key, image_policy).await
}

/// Like [`run`], but with an explicit bind address (`--host`).
pub async fn run_on(
    host: &str,
    port: u16,
    api_key: Option<String>,
    image_policy: crate::text::ImagePolicy,
) -> anyhow::Result<()> {
    // Blank values (docker-compose passes one when the variable is unset)
    // mean "not configured".
    let api_key = non_blank(api_key);
    let ip: std::net::IpAddr = host
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid --host '{host}': expected an IP address"))?;
    let font_db = render::build_font_db(None)?;

    let template_count = template::list_templates(std::path::Path::new("./templates")).len();

    let auth_enabled = api_key.is_some();
    if auth_enabled {
        log::info!("Authentication enabled (bearer token required)");
    } else {
        log::warn!("Authentication disabled — set COSY_API_KEY to secure the API");
        if !ip.is_loopback() {
            log::warn!(
                "Listening on {ip} without authentication — anyone who can reach this port can render; use --host 127.0.0.1 for local dev"
            );
        }
    }

    let max_concurrent = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(2);
    let state = Arc::new(AppState {
        font_db,
        template_count,
        render_slots: tokio::sync::Semaphore::new(max_concurrent),
        image_policy,
        api_key: api_key.clone(),
        signing_key: api_key.clone(),
    });

    // Protected routes require auth
    let protected = Router::new()
        .route("/api/templates", get(list_templates))
        .route("/api/render", post(render_handler))
        .layer(axum::middleware::from_fn_with_state(
            api_key,
            auth_middleware,
        ));

    let app = Router::new()
        // Health is always public (for Docker healthcheck)
        .route("/api/health", get(health))
        .merge(protected)
        // Signed GET render URLs (disabled without a signing key)
        .route("/r/{template}", get(signed_render_handler))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::new(ip, port);
    log::info!("Cosy API server listening on http://{}", addr);
    log::info!("Loaded {} templates", template_count);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ─── Auth Middleware ───────────────────────────────────────────────

/// Middleware that checks `Authorization: Bearer <token>` header.
/// If no API key is configured, the middleware passes through (dev mode).
async fn auth_middleware(
    State(expected_key): State<Option<String>>,
    req: Request,
    next: Next,
) -> Response {
    let Some(expected) = expected_key else {
        // No API key configured — auth disabled
        return next.run(req).await;
    };

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    match auth_header {
        Some(header_val) if header_val.starts_with("Bearer ") => {
            let token = &header_val[7..];
            // Constant-time comparison to prevent timing attacks
            if constant_time_eq(token.as_bytes(), expected.as_bytes()) {
                next.run(req).await
            } else {
                error_response(StatusCode::UNAUTHORIZED, "Invalid API key".into())
            }
        }
        _ => error_response(
            StatusCode::UNAUTHORIZED,
            "Missing or invalid Authorization header. Expected: Bearer <token>".into(),
        ),
    }
}

/// Constant-time byte comparison to prevent timing side-channel attacks.
/// Compares all bytes regardless of match position, accumulating differences.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

// ─── Handlers ──────────────────────────────────────────────────────

async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        templates: state.template_count,
        auth_enabled: state.api_key.is_some(),
    })
}

async fn list_templates() -> Json<Vec<crate::schema::TemplateDef>> {
    // Reads every schema.json from disk — keep it off the async workers.
    let templates = tokio::task::spawn_blocking(|| {
        template::list_templates(std::path::Path::new("./templates"))
    })
    .await
    .unwrap_or_default();
    Json(templates)
}

async fn render_handler(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RenderRequest>,
) -> Response {
    let format = req.response_format.unwrap_or(ResponseFormat::Png);
    log::info!(
        "Render request: template={}, slides={}, scale={}, format={:?}",
        req.template,
        req.data.slides.len(),
        req.scale,
        format
    );

    // Metadata passthrough: size-cap so callers can't smuggle unbounded
    // payloads through the echo field.
    if let Some(meta) = &req.metadata {
        let serialized = match serde_json::to_string(meta) {
            Ok(s) => s,
            Err(e) => {
                return error_response(
                    StatusCode::BAD_REQUEST,
                    format!("metadata is not valid JSON: {e}"),
                );
            }
        };
        if serialized.len() > MAX_METADATA_BYTES {
            return error_response(
                StatusCode::PAYLOAD_TOO_LARGE,
                format!(
                    "metadata too large: {} bytes (max {})",
                    serialized.len(),
                    MAX_METADATA_BYTES
                ),
            );
        }
    }

    if req.data.slides.is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "Input data must contain at least one slide".into(),
        );
    }

    if !req.scale.is_finite() || !(MIN_SCALE..=MAX_SCALE).contains(&req.scale) {
        return error_response(
            StatusCode::BAD_REQUEST,
            format!("scale must be between {MIN_SCALE} and {MAX_SCALE}"),
        );
    }

    if !is_safe_template_name(&req.template) {
        return error_response(
            StatusCode::BAD_REQUEST,
            "Template error: template must be a plain template name (letters, digits, '-', '_')"
                .into(),
        );
    }

    // Load template definition
    let tmpl = match template::load_template(&req.template) {
        Ok(t) => t,
        Err(e) => {
            return error_response(StatusCode::BAD_REQUEST, format!("Template error: {e:#}"));
        }
    };

    // Find template directory
    let template_dir = match template::find_template_dir_for(&req.template) {
        Ok(d) => d,
        Err(e) => {
            return error_response(StatusCode::NOT_FOUND, format!("Template not found: {e:#}"));
        }
    };

    // Same schema validation as the CLI and signed GET route.
    let errors = template::validate_input(&tmpl, &req.data);
    if !errors.is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            format!("validation failed: {}", errors.join("; ")),
        );
    }

    if format == ResponseFormat::Json && req.data.slides.len() > MAX_SLIDES_PER_REQUEST {
        return error_response(
            StatusCode::PAYLOAD_TOO_LARGE,
            format!(
                "too many slides: {} (max {} per request)",
                req.data.slides.len(),
                MAX_SLIDES_PER_REQUEST
            ),
        );
    }

    // Which slides to render: all for json format, one for image format.
    let slide_indices: Vec<usize> = match format {
        ResponseFormat::Json => (0..req.data.slides.len()).collect(),
        ResponseFormat::Png => {
            let idx = req.slide_index.unwrap_or(0);
            if idx >= req.data.slides.len() {
                return error_response(
                    StatusCode::BAD_REQUEST,
                    format!(
                        "slide_index {} out of range (input has {} slide(s))",
                        idx,
                        req.data.slides.len()
                    ),
                );
            }
            vec![idx]
        }
    };

    // Blocking work (template IO, resvg, possibly remote image fetches) runs
    // on the blocking thread pool so the async runtime is never blocked.
    let font_db = Arc::clone(&state.font_db);
    let image_policy = state.image_policy;
    let scale = req.scale;
    let data = req.data;
    let template_id = tmpl.id.clone();
    let dims = tmpl.dimensions.clone();
    let image_format: crate::format::OutputFormat =
        req.image_format.map(Into::into).unwrap_or_default();
    let out_w = (dims.width as f32 * scale).round() as u32;
    let out_h = (dims.height as f32 * scale).round() as u32;
    let _permit = state
        .render_slots
        .acquire()
        .await
        .expect("render semaphore is never closed");
    let render_result = tokio::task::spawn_blocking(move || {
        slide_indices
            .into_iter()
            .map(|i| {
                if image_format.is_pixel_based() {
                    let png = render::render_slide_to_pixels(
                        &tmpl,
                        &template_dir,
                        &data,
                        i,
                        scale,
                        &font_db,
                        image_policy,
                    )?;
                    let bytes = image_format.encode_owned(png, out_w, out_h)?;
                    Ok((i, bytes))
                } else {
                    // SVG: vector path — scale does not apply.
                    let svg = render::render_slide_to_svg(
                        &tmpl,
                        &template_dir,
                        &data,
                        i,
                        &font_db,
                        image_policy,
                    )?;
                    Ok((i, svg.into_bytes()))
                }
            })
            .collect::<anyhow::Result<Vec<(usize, Vec<u8>)>>>()
    })
    .await;

    match render_result {
        Ok(Ok(rendered)) => match format {
            ResponseFormat::Png => {
                let (_, image_bytes) = rendered.into_iter().next().expect("one slide rendered");
                if let Some(meta) = &req.metadata {
                    log::info!("Render metadata: {meta}");
                }
                log::info!(
                    "Rendered {} bytes of {}",
                    image_bytes.len(),
                    image_format.mime_type()
                );
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, image_format.mime_type())],
                    image_bytes,
                )
                    .into_response()
            }
            ResponseFormat::Json => {
                let slides_json: Vec<RenderedSlide> = rendered
                    .into_iter()
                    .map(|(i, bytes)| RenderedSlide {
                        index: i,
                        png_base64: base64::Engine::encode(
                            &base64::engine::general_purpose::STANDARD,
                            &bytes,
                        ),
                        image_format: image_format.extension(),
                    })
                    .collect();
                log::info!("Rendered {} slide(s) as JSON", slides_json.len());
                (
                    StatusCode::OK,
                    Json(RenderResponse {
                        template: template_id,
                        slides: slides_json.len(),
                        width: (dims.width as f32 * scale) as u32,
                        height: (dims.height as f32 * scale) as u32,
                        data: slides_json,
                        metadata: req.metadata,
                    }),
                )
                    .into_response()
            }
        },
        Ok(Err(e)) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Render error: {e:#}"),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Render worker failed: {e}"),
        ),
    }
}

/// Build a JSON error response.
fn error_response(status: StatusCode, message: String) -> Response {
    (status, Json(ErrorResponse { error: message })).into_response()
}

// ─── GET /r/:template — signed render URLs (#95) ─────────────────────

/// Render via a signed GET URL, for `<meta property="og:image">` style use:
/// blog engines embed a plain URL with no client library and no POST.
///
/// Path: `/r/{template}.{ext}` — ext selects the container (`png`|`webp`).
/// Query: `d` = base64url(JSON data), `sig` = HMAC-SHA256 hex of
/// `{template}:{d}` under the signing key (= API key). Responses carry
/// `Cache-Control: public, max-age=3600` for OG re-crawls.
async fn signed_render_handler(
    State(state): State<Arc<AppState>>,
    Path(template_with_ext): Path<String>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Response {
    // 1. Feature gate: no signing key configured → route disabled (dev mode
    //    keeps unsigned POST only, per the issue's security note).
    let Some(signing_key) = state.signing_key.clone() else {
        return error_response(
            StatusCode::NOT_FOUND,
            "signed GET rendering is disabled (no API key / signing key configured)".into(),
        );
    };

    // 2. Split template.ext
    let (template_id, image_format) = match template_with_ext.rsplit_once('.') {
        Some((t, "png")) => (t.to_string(), crate::format::OutputFormat::Png),
        Some((t, "webp")) => (t.to_string(), crate::format::OutputFormat::WebP),
        Some((t, _)) => {
            return error_response(
                StatusCode::BAD_REQUEST,
                format!("unsupported extension in '{t}.': use .png or .webp"),
            );
        }
        None => {
            return error_response(
                StatusCode::BAD_REQUEST,
                "path must be /r/{template}.{png|webp}".into(),
            );
        }
    };

    // 3. Required query params
    let Some(d) = params.get("d") else {
        return error_response(StatusCode::BAD_REQUEST, "missing d query parameter".into());
    };
    let Some(sig) = params.get("sig") else {
        return error_response(
            StatusCode::BAD_REQUEST,
            "missing sig query parameter".into(),
        );
    };

    // 4. Verify signature + decode payload (also rejects >8 KB payloads)
    let data = match verify_signed_request(&signing_key, &template_id, d, sig) {
        Ok(data) => data,
        Err((status, msg)) => return error_response(status, msg),
    };

    if data.slides.is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "Input data must contain at least one slide".into(),
        );
    }

    // 5. Load template + dir (error mapping mirrors render_handler)
    if !is_safe_template_name(&template_id) {
        return error_response(
            StatusCode::BAD_REQUEST,
            "Template error: invalid template name".into(),
        );
    }
    let Ok(tmpl) = template::load_template(&template_id) else {
        return error_response(
            StatusCode::BAD_REQUEST,
            format!("Template error: template '{template_id}' not found"),
        );
    };
    let Ok(template_dir) = template::find_template_dir_for(&template_id) else {
        return error_response(
            StatusCode::NOT_FOUND,
            format!("Template not found: '{template_id}'"),
        );
    };

    let errors = crate::template::validate_input(&tmpl, &data);
    if !errors.is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            format!("validation failed: {}", errors.join("; ")),
        );
    }

    // 6. Render first slide on the blocking pool
    let font_db = Arc::clone(&state.font_db);
    let image_policy = state.image_policy;
    let scale = 1.0_f32;
    let dims = tmpl.dimensions.clone();
    let _permit = state
        .render_slots
        .acquire()
        .await
        .expect("render semaphore is never closed");
    let render_result = tokio::task::spawn_blocking(move || {
        let pixels = render::render_slide_to_pixels(
            &tmpl,
            &template_dir,
            &data,
            0,
            scale,
            &font_db,
            image_policy,
        )?;
        let w = (dims.width as f32 * scale).round() as u32;
        let h = (dims.height as f32 * scale).round() as u32;
        image_format.encode_owned(pixels, w, h)
    })
    .await;

    match render_result {
        Ok(Ok(bytes)) => {
            log::info!(
                "Signed GET render: template={} format={} bytes={}",
                template_id,
                image_format.extension(),
                bytes.len()
            );
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, image_format.mime_type()),
                    (header::CACHE_CONTROL, "public, max-age=3600"),
                ],
                bytes,
            )
                .into_response()
        }
        Ok(Err(e)) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Render error: {e:#}"),
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Render worker failed: {e}"),
        ),
    }
}
