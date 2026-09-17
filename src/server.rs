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
    extract::{Request, State},
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
    font_db: usvg::fontdb::Database,
    image_policy: crate::text::ImagePolicy,
    api_key: Option<String>,
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
    /// Base64-encoded PNG bytes.
    pub png_base64: String,
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

/// Start the HTTP server.
///
/// If `api_key` is Some, all endpoints except /api/health require
/// `Authorization: Bearer <api_key>` header.
pub async fn run(
    port: u16,
    api_key: Option<String>,
    image_policy: crate::text::ImagePolicy,
) -> anyhow::Result<()> {
    let font_db = render::build_font_db(None)?;

    let template_count = template::list_templates(std::path::Path::new("./templates")).len();

    let auth_enabled = api_key.is_some();
    if auth_enabled {
        log::info!("Authentication enabled (bearer token required)");
    } else {
        log::warn!("Authentication disabled — set COSY_API_KEY to secure the API");
    }

    let state = Arc::new(AppState {
        font_db,
        image_policy,
        api_key: api_key.clone(),
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
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
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
    let templates = template::list_templates(std::path::Path::new("./templates"));
    let _ = &state; // font_db ready
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        templates: templates.len(),
        auth_enabled: state.api_key.is_some(),
    })
}

async fn list_templates() -> Json<Vec<crate::schema::TemplateDef>> {
    let templates = template::list_templates(std::path::Path::new("./templates"));
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

    if req.data.slides.is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "Input data must contain at least one slide".into(),
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

    // Which slides to render: all for json format, one for png format.
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
    let font_db = state.font_db.clone();
    let image_policy = state.image_policy;
    let scale = req.scale;
    let data = req.data;
    let template_id = tmpl.id.clone();
    let dims = tmpl.dimensions.clone();
    let render_result = tokio::task::spawn_blocking(move || {
        slide_indices
            .into_iter()
            .map(|i| {
                let png = render::render_slide_to_png(
                    &tmpl,
                    &template_dir,
                    &data,
                    i,
                    scale,
                    &font_db,
                    image_policy,
                )?;
                Ok((i, png))
            })
            .collect::<anyhow::Result<Vec<(usize, Vec<u8>)>>>()
    })
    .await;

    match render_result {
        Ok(Ok(rendered)) => match format {
            ResponseFormat::Png => {
                let (_, png_bytes) = rendered.into_iter().next().expect("one slide rendered");
                log::info!("Rendered {} bytes of PNG", png_bytes.len());
                (
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "image/png")],
                    png_bytes,
                )
                    .into_response()
            }
            ResponseFormat::Json => {
                let slides_json: Vec<RenderedSlide> = rendered
                    .into_iter()
                    .map(|(i, png)| RenderedSlide {
                        index: i,
                        png_base64: base64::Engine::encode(
                            &base64::engine::general_purpose::STANDARD,
                            &png,
                        ),
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
