//! Core rendering pipeline: JSON data → minijinja → SVG → resvg → PNG.
//!
//! This module wires together the full pipeline:
//! 1. Load template definition + SVG template
//! 2. Load input data (brand + slides)
//! 3. For each slide: process_template → SVG string → usvg::Tree → resvg → PNG
//! 4. Write PNG file(s)

use crate::format::OutputFormat;
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
/// If output has an extension (e.g. `slide.png`), renders a single image.
/// If output is a directory, renders each slide as `{NN}.<ext>`.
/// Slides are always encoded as PNG internally; `format` only selects the
/// container written to disk (see [`OutputFormat`]).
pub fn render_template_data(
    template_name: &str,
    data: &InputData,
    output: &Path,
    scale: f32,
    font_dir: Option<&Path>,
    image_policy: crate::text::ImagePolicy,
    format: OutputFormat,
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
        // Single image output
        if format.is_pixel_based() {
            let png = render_slide(
                &template,
                &template_dir,
                data,
                0,
                scale,
                &font_db,
                image_policy,
            )?;
            let bytes = format.encode(
                &png,
                output_width(&template.dimensions, scale),
                output_height(&template.dimensions, scale),
            )?;
            // The -o path is used verbatim (no extension rewriting) — existing
            // scripts that render to exact paths stay byte-path compatible.
            // Recommend matching the extension to --format for clarity.
            ensure_parent_dir(output)?;
            std::fs::write(output, &bytes)?;
            log::info!("Written: {}", output.display());
            output_files.push(output.to_string_lossy().to_string());
        } else {
            // SVG: vector path — scale does not apply, write the resolved tree.
            let svg =
                render_slide_to_svg(&template, &template_dir, data, 0, &font_db, image_policy)?;
            ensure_parent_dir(output)?;
            std::fs::write(output, &svg)?;
            log::info!("Written: {}", output.display());
            output_files.push(output.to_string_lossy().to_string());
        }
    } else {
        // Multi-slide output to directory
        std::fs::create_dir_all(output)?;
        for i in 0..data.slides.len() {
            let filename = format!("{:02}.{}", i + 1, format.extension());
            let path: PathBuf = output.join(&filename);
            if format.is_pixel_based() {
                let png = render_slide(
                    &template,
                    &template_dir,
                    data,
                    i,
                    scale,
                    &font_db,
                    image_policy,
                )?;
                let bytes = format.encode(
                    &png,
                    output_width(&template.dimensions, scale),
                    output_height(&template.dimensions, scale),
                )?;
                std::fs::write(&path, &bytes)?;
            } else {
                let svg =
                    render_slide_to_svg(&template, &template_dir, data, i, &font_db, image_policy)?;
                std::fs::write(&path, &svg)?;
            }
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
    image_policy: crate::text::ImagePolicy,
    format: OutputFormat,
) -> anyhow::Result<RenderResult> {
    let data = InputData::from_file(data_path)?;
    log::info!("Loaded data: {} slide(s)", data.slides.len());
    render_template_data(
        template_name,
        &data,
        output,
        scale,
        font_dir,
        image_policy,
        format,
    )
}

/// Final pixel width after applying the scale factor.
fn output_width(dims: &crate::schema::Dimensions, scale: f32) -> u32 {
    (dims.width as f32 * scale).round() as u32
}

/// Final pixel height after applying the scale factor.
fn output_height(dims: &crate::schema::Dimensions, scale: f32) -> u32 {
    (dims.height as f32 * scale).round() as u32
}

// ─── Render Single Slide ────────────────────────────────────────────

/// Render a single slide to raw RGBA pixels.
fn render_slide(
    template: &TemplateDef,
    template_dir: &Path,
    data: &InputData,
    slide_index: usize,
    scale: f32,
    font_db: &usvg::fontdb::Database,
    image_policy: crate::text::ImagePolicy,
) -> anyhow::Result<Vec<u8>> {
    render_slide_to_pixels(
        template,
        template_dir,
        data,
        slide_index,
        scale,
        font_db,
        image_policy,
    )
}

/// Render a single slide to raw RGBA8 pixels (public API for
/// format-aware callers — pair with `OutputFormat::encode`).
pub fn render_slide_to_pixels(
    template: &TemplateDef,
    template_dir: &Path,
    data: &InputData,
    slide_index: usize,
    scale: f32,
    font_db: &usvg::fontdb::Database,
    image_policy: crate::text::ImagePolicy,
) -> anyhow::Result<Vec<u8>> {
    let slide_data = &data.slides[slide_index];

    // 1. Process minijinja template → SVG string
    let svg_string = crate::template::process_template(
        template,
        template_dir,
        &data.brand,
        slide_data,
        image_policy,
    )?;

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

    // 5. Return raw RGBA pixels for container-format encoding
    Ok(pixmap.data().to_vec())
}

/// Render a single slide to a self-contained SVG string (public API for
/// the SVG output format).
///
/// Returns the usvg-resolved tree serialized with text converted to paths
/// (default `preserve_text: false`), so the output has zero font
/// dependencies and renders identically anywhere. `scale` is a raster
/// concept and does not apply — the `viewBox` equals the template canvas.
/// Background images/logo are already inlined as data URIs by the template
/// processor, so no external references remain.
pub fn render_slide_to_svg(
    template: &TemplateDef,
    template_dir: &Path,
    data: &InputData,
    slide_index: usize,
    font_db: &usvg::fontdb::Database,
    image_policy: crate::text::ImagePolicy,
) -> anyhow::Result<String> {
    let slide_data = &data.slides[slide_index];

    // 1. Process minijinja template → SVG string
    let svg_string = crate::template::process_template(
        template,
        template_dir,
        &data.brand,
        slide_data,
        image_policy,
    )?;

    // 2. Resolve the SVG through usvg (same as the raster path: applies
    //    the font DB, resolves hrefs, normalizes the tree)…
    let opts = usvg::Options {
        font_family: "Inter".to_string(),
        fontdb: std::sync::Arc::new(font_db.clone()),
        ..Default::default()
    };
    let tree = usvg::Tree::from_str(&svg_string, &opts)?;

    // 3. …then serialize with text converted to outlines.
    let write_opts = usvg::WriteOptions {
        preserve_text: false,
        ..Default::default()
    };
    Ok(tree.to_string(&write_opts))
}

/// Render a single slide to PNG bytes (legacy convenience wrapper).
pub fn render_slide_to_png(
    template: &TemplateDef,
    template_dir: &Path,
    data: &InputData,
    slide_index: usize,
    scale: f32,
    font_db: &usvg::fontdb::Database,
    image_policy: crate::text::ImagePolicy,
) -> anyhow::Result<Vec<u8>> {
    let pixels = render_slide_to_pixels(
        template,
        template_dir,
        data,
        slide_index,
        scale,
        font_db,
        image_policy,
    )?;
    let w = (template.dimensions.width as f32 * scale).round() as u32;
    let h = (template.dimensions.height as f32 * scale).round() as u32;
    OutputFormat::Png.encode(&pixels, w, h)
}

// ─── Font Database ──────────────────────────────────────────────────

/// Build a font database from bundled fonts + system fonts + optional custom directory.
///
/// Load order (later loads override earlier for same family):
/// 1. Bundled fonts (Inter Regular/Bold/SemiBold, JetBrains Mono Regular)
/// 2. System fonts
/// 3. User-specified --font-dir
pub fn build_font_db(custom_dir: Option<&Path>) -> anyhow::Result<usvg::fontdb::Database> {
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
        "Inter",
        "Italic",
        include_bytes!("assets/fonts/Inter-Italic.ttf"),
    );
    load_bundled_font(
        &mut db,
        "Inter",
        "Bold Italic",
        include_bytes!("assets/fonts/Inter-BoldItalic.ttf"),
    );
    load_bundled_font(
        &mut db,
        "Inter",
        "Black",
        include_bytes!("assets/fonts/Inter-Black.ttf"),
    );
    load_bundled_font(
        &mut db,
        "JetBrains Mono",
        "Regular",
        include_bytes!("assets/fonts/JetBrainsMono-Regular.ttf"),
    );
    // Display font: Space Grotesk (trending modern geometric sans-serif)
    load_bundled_font(
        &mut db,
        "Space Grotesk",
        "Medium",
        include_bytes!("assets/fonts/SpaceGrotesk-Medium.ttf"),
    );
    load_bundled_font(
        &mut db,
        "Space Grotesk",
        "SemiBold",
        include_bytes!("assets/fonts/SpaceGrotesk-SemiBold.ttf"),
    );
    load_bundled_font(
        &mut db,
        "Space Grotesk",
        "Bold",
        include_bytes!("assets/fonts/SpaceGrotesk-Bold.ttf"),
    );
    // Handwritten fonts: Kalam (body) + Caveat (display) for notebook-style templates
    load_bundled_font(
        &mut db,
        "Kalam",
        "Light",
        include_bytes!("assets/fonts/Kalam-Light.ttf"),
    );
    load_bundled_font(
        &mut db,
        "Kalam",
        "Regular",
        include_bytes!("assets/fonts/Kalam-Regular.ttf"),
    );
    load_bundled_font(
        &mut db,
        "Kalam",
        "Bold",
        include_bytes!("assets/fonts/Kalam-Bold.ttf"),
    );
    load_bundled_font(
        &mut db,
        "Caveat",
        "Medium",
        include_bytes!("assets/fonts/Caveat-Medium.ttf"),
    );
    load_bundled_font(
        &mut db,
        "Caveat",
        "Bold",
        include_bytes!("assets/fonts/Caveat-Bold.ttf"),
    );
    log::debug!(
        "Loaded 15 bundled fonts (Inter R/B/SB/Black/I/BI, JetBrains Mono R, SpaceGrotesk M/SB/B, Kalam L/R/B, Caveat M/B)"
    );

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
