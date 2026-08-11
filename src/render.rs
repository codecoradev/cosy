//! Core rendering pipeline: JSON data → minijinja → SVG → resvg → PNG.
//!
//! This module wires together the full pipeline:
//! 1. Load template definition + SVG template
//! 2. Load input data (brand + slides)
//! 3. For each slide: process_template → SVG string → usvg::Tree → resvg → PNG
//! 4. Write PNG file(s)

use crate::schema::{InputData, TemplateDef};
use std::path::{Path, PathBuf};
use std::time::Instant;

// ─── Public API ─────────────────────────────────────────────────────

/// Result of a render operation — used by --json-output.
#[derive(Debug, serde::Serialize)]
pub struct RenderResult {
    pub template: String,
    pub files: Vec<String>,
    pub render_time_ms: u128,
    pub slides: usize,
}

/// Render a template with pre-loaded data to output file(s).
///
/// If output has an extension (e.g. `slide.png`), renders a single PNG.
/// If output is a directory, renders each slide as `{NN}.png`.
pub fn render_template_data(
    template_name: &str,
    data: &InputData,
    output: &Path,
    scale: f32,
    font_dir: Option<&Path>,
) -> anyhow::Result<RenderResult> {
    let start = Instant::now();

    // 1. Load template
    let template = crate::template::load_template(template_name)?;
    log::info!(
        "Loaded template: {} ({}×{})",
        template.name,
        template.dimensions.width,
        template.dimensions.height
    );

    // 2. Locate template directory for SVG file
    let template_dir = crate::template::find_template_dir_for(template_name)?;

    // 3. Build font database
    let font_db = build_font_db(font_dir)?;

    // 4. Render slides
    let mut output_files = Vec::new();

    if data.is_single_slide() || output.extension().is_some() {
        // Single PNG output
        let png = render_slide(&template, &template_dir, data, 0, scale, &font_db)?;
        ensure_parent_dir(output)?;
        std::fs::write(output, &png)?;
        log::info!("Written: {}", output.display());
        output_files.push(output.to_string_lossy().to_string());
    } else {
        // Multi-slide output to directory
        std::fs::create_dir_all(output)?;
        for i in 0..data.slides.len() {
            let png = render_slide(&template, &template_dir, data, i, scale, &font_db)?;
            let filename = format!("{:02}.png", i + 1);
            let path: PathBuf = output.join(&filename);
            std::fs::write(&path, &png)?;
            log::info!("Written: {}", path.display());
            output_files.push(path.to_string_lossy().to_string());
        }
        log::info!(
            "Rendered {} slide(s) to {}",
            data.slides.len(),
            output.display()
        );
    }

    let elapsed = start.elapsed().as_millis();
    log::info!("Total render time: {}ms", elapsed);

    Ok(RenderResult {
        template: template_name.to_string(),
        files: output_files,
        render_time_ms: elapsed,
        slides: data.slides.len(),
    })
}

/// Convenience wrapper: load data from file, then render.
pub fn render_template(
    template_name: &str,
    data_path: &str,
    output: &Path,
    scale: f32,
    font_dir: Option<&Path>,
) -> anyhow::Result<RenderResult> {
    let data = InputData::from_file(data_path)?;
    log::info!("Loaded data: {} slide(s)", data.slides.len());
    render_template_data(template_name, &data, output, scale, font_dir)
}

// ─── Render Single Slide ────────────────────────────────────────────

/// Render a single slide to PNG bytes.
fn render_slide(
    template: &TemplateDef,
    template_dir: &Path,
    data: &InputData,
    slide_index: usize,
    scale: f32,
    font_db: &usvg::fontdb::Database,
) -> anyhow::Result<Vec<u8>> {
    let slide_data = &data.slides[slide_index];

    // 1. Process minijinja template → SVG string
    let svg_string =
        crate::template::process_template(template, template_dir, &data.brand, slide_data)?;

    log::debug!("SVG generated: {} bytes", svg_string.len());

    // 2. Parse SVG via usvg with font database
    let opts = usvg::Options {
        font_family: "Inter".to_string(),
        fontdb: std::sync::Arc::new(font_db.clone()),
        ..Default::default()
    };

    let tree = usvg::Tree::from_str(&svg_string, &opts)?;

    // 3. Create pixmap at scaled resolution
    let base_w = template.dimensions.width;
    let base_h = template.dimensions.height;
    let out_w = (base_w as f32 * scale).round() as u32;
    let out_h = (base_h as f32 * scale).round() as u32;

    let mut pixmap = tiny_skia::Pixmap::new(out_w, out_h)
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap {}×{}", out_w, out_h))?;

    // 4. Render via resvg with scale transform
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // 5. Encode to PNG
    Ok(pixmap.encode_png()?)
}

// ─── Font Database ──────────────────────────────────────────────────

/// Build a font database from bundled fonts + system fonts + optional custom directory.
///
/// Load order (later loads override earlier for same family):
/// 1. Bundled fonts (Inter Regular/Bold/SemiBold, JetBrains Mono Regular)
/// 2. System fonts
/// 3. User-specified --font-dir
fn build_font_db(custom_dir: Option<&Path>) -> anyhow::Result<usvg::fontdb::Database> {
    let mut db = usvg::fontdb::Database::new();

    // 1. Load bundled fonts (embedded at compile time)
    load_bundled_font(
        &mut db,
        "Inter",
        "Regular",
        include_bytes!("assets/fonts/Inter-Regular.ttf"),
    );
    load_bundled_font(
        &mut db,
        "Inter",
        "Bold",
        include_bytes!("assets/fonts/Inter-Bold.ttf"),
    );
    load_bundled_font(
        &mut db,
        "Inter",
        "SemiBold",
        include_bytes!("assets/fonts/Inter-SemiBold.ttf"),
    );
    load_bundled_font(
        &mut db,
        "JetBrains Mono",
        "Regular",
        include_bytes!("assets/fonts/JetBrainsMono-Regular.ttf"),
    );
    log::debug!("Loaded 4 bundled fonts (Inter R/B/SB, JetBrains Mono R)");

    // 2. Load system fonts (may fail in containers — that's OK)
    db.load_system_fonts();

    // Set default sans-serif family
    db.set_sans_serif_family("Inter");

    // 3. Load custom font directory if specified
    if let Some(dir) = custom_dir {
        if dir.is_dir() {
            db.load_fonts_dir(dir);
            log::info!("Loaded custom fonts from: {}", dir.display());
        } else {
            log::warn!("Font directory not found: {}", dir.display());
        }
    }

    Ok(db)
}

/// Register a bundled font from embedded bytes.
fn load_bundled_font(db: &mut usvg::fontdb::Database, family: &str, _weight: &str, bytes: &[u8]) {
    db.load_font_data(bytes.to_vec());
    log::trace!("Registered bundled font: {} ({})", family, _weight);
}

// ─── Helpers ────────────────────────────────────────────────────────

/// Ensure parent directory exists for a file path.
fn ensure_parent_dir(path: &Path) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}
