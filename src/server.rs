//! HTTP API server for Cosy.
//!
//! Endpoints:
//! - GET  /api/health     — health check
//! - GET  /api/templates  — list all templates
//! - POST /api/render     — render template with JSON data → PNG

use crate::render;
use crate::schema::InputData;
use crate::template;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;

/// Shared server state — font DB built once at startup.
struct AppState {
    font_db: usvg::fontdb::Database,
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
}

fn default_scale() -> f32 {
    2.0
}

/// Response for GET /api/health.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub templates: usize,
}

/// Response wrapper for errors.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Start the HTTP server.
pub async fn run(port: u16) -> anyhow::Result<()> {
    let font_db = render::build_font_db(None)?;

    let template_count = template::list_templates(std::path::Path::new("./templates")).len();

    let state = Arc::new(AppState { font_db });

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/templates", get(list_templates))
        .route("/api/render", post(render_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    log::info!("Cosy API server listening on http://{}", addr);
    log::info!("Loaded {} templates", template_count);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// ─── Handlers ──────────────────────────────────────────────────────

async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let templates = template::list_templates(std::path::Path::new("./templates"));
    let _ = &state; // font_db ready
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        templates: templates.len(),
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
    log::info!(
        "Render request: template={}, slides={}, scale={}",
        req.template,
        req.data.slides.len(),
        req.scale
    );

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

    // Render first slide (API returns single PNG for simplicity)
    match render::render_slide_to_png(
        &tmpl,
        &template_dir,
        &req.data,
        0,
        req.scale,
        &state.font_db,
    ) {
        Ok(png_bytes) => {
            log::info!("Rendered {} bytes of PNG", png_bytes.len());
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/png")],
                png_bytes,
            )
                .into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Render error: {e:#}"),
        ),
    }
}

/// Build a JSON error response.
fn error_response(status: StatusCode, message: String) -> Response {
    (status, Json(ErrorResponse { error: message })).into_response()
}
