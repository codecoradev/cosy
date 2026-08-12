//! Template loading, validation, and minijinja processing.
//!
//! Templates live in `./templates/{name}/` and contain:
//! - `schema.json`   — field definitions (brand + slide)
//! - `template.svg`  — minijinja-powered SVG template
//! - `defaults.json` — optional sample data for testing

use crate::schema::{FieldType, InputData, TemplateDef};
use std::path::{Path, PathBuf};

// ─── Template Discovery ─────────────────────────────────────────────

/// Load a template definition by name or path.
pub fn load_template(name: &str) -> anyhow::Result<TemplateDef> {
    let dir = find_template_dir_for(name)?;
    let schema_path = dir.join("schema.json");
    let schema_str = std::fs::read_to_string(&schema_path)?;
    let def: TemplateDef = serde_json::from_str(&schema_str)?;
    Ok(def)
}

/// Find the template directory.
/// Checks: direct path → `./templates/{name}/`
pub fn find_template_dir_for(name: &str) -> anyhow::Result<PathBuf> {
    // Direct path
    let direct = PathBuf::from(name);
    if direct.is_dir() {
        return Ok(direct);
    }

    // ./templates/{name}/
    let template_dir = PathBuf::from("./templates").join(name);
    if template_dir.is_dir() {
        return Ok(template_dir);
    }

    anyhow::bail!(
        "Template '{}' not found. Looked in ./templates/{}",
        name,
        name
    )
}

/// Load the raw SVG template string.
pub fn load_svg(template_dir: &Path) -> anyhow::Result<String> {
    // Prefer .svg.j2, fallback to .svg
    let j2 = template_dir.join("template.svg.j2");
    let svg = template_dir.join("template.svg");
    let path = if j2.exists() { &j2 } else { &svg };

    if !path.exists() {
        anyhow::bail!(
            "No template file found in {}. Expected template.svg.j2 or template.svg",
            template_dir.display()
        );
    }

    Ok(std::fs::read_to_string(path)?)
}

// ─── minijinja Processing ───────────────────────────────────────────

/// Apply minijinja token replacement to the SVG template.
///
/// Context includes:
/// - `brand.*` — global brand fields
/// - `slide.*` — per-slide content
/// - `{field}_lines` — pre-wrapped text lines for text fields with max
/// - `logo_data_uri` — base64 data URI if brand.logo path is set
pub fn process_template(
    template: &TemplateDef,
    template_dir: &Path,
    brand: &serde_json::Value,
    slide: &serde_json::Value,
) -> anyhow::Result<String> {
    let svg_template = load_svg(template_dir)?;

    // Build minijinja environment with custom filters
    let mut env = minijinja::Environment::new();
    env.add_template("slide", &svg_template)?;

    // Register custom filters
    env.add_filter("wordwrap", filter_wordwrap);
    env.add_filter("b64", filter_b64);

    // Build context
    let mut context = serde_json::Map::new();

    // Insert brand fields under `brand.*`
    context.insert("brand".into(), brand.clone());

    // Insert slide fields under `slide.*`
    context.insert("slide".into(), slide.clone());

    // Also flatten brand + slide to top-level for convenience
    if let serde_json::Value::Object(ref brand_map) = brand {
        for (k, v) in brand_map {
            context.insert(k.clone(), v.clone());
        }
    }
    if let serde_json::Value::Object(ref slide_map) = slide {
        for (k, v) in slide_map {
            context.insert(k.clone(), v.clone());
        }
    }

    // Pre-wrap text fields that have a max chars limit
    for (field_name, field_spec) in &template.slide_fields {
        if field_spec.field_type == FieldType::Text {
            // Use wrap_width for visual line breaking, fallback to max
            let wrap = field_spec.wrap_width.or(field_spec.max);
            if let Some(wrap_chars) = wrap {
                if let Some(text) = slide.get(field_name).and_then(|v| v.as_str()) {
                    let wrapped = crate::text::wrap_text(text, wrap_chars);
                    context.insert(
                        format!("{}_lines", field_name),
                        serde_json::Value::Array(
                            wrapped
                                .iter()
                                .map(|l| serde_json::Value::String(l.clone()))
                                .collect(),
                        ),
                    );
                }
            }
        }
    }

    // Convert logo path to data URI if present
    if let Some(logo_path) = brand.get("logo").and_then(|v| v.as_str()) {
        if !logo_path.is_empty() {
            match crate::text::image_to_data_uri(logo_path) {
                Ok(data_uri) => {
                    context.insert("logo_data_uri".into(), serde_json::Value::String(data_uri));
                }
                Err(e) => {
                    log::warn!("Failed to load logo '{}': {}", logo_path, e);
                }
            }
        }
    }

    // Convert bg_image path to data URI if present (brand-level or slide-level)
    let bg_image = brand
        .get("bg_image")
        .or_else(|| slide.get("bg_image"))
        .and_then(|v| v.as_str());
    if let Some(bg_path) = bg_image {
        if !bg_path.is_empty() {
            match crate::text::image_to_data_uri(bg_path) {
                Ok(data_uri) => {
                    context.insert(
                        "bg_image_data_uri".into(),
                        serde_json::Value::String(data_uri),
                    );
                }
                Err(e) => {
                    log::warn!("Failed to load bg_image '{}': {}", bg_path, e);
                }
            }
        }
    }

    // Render
    let tmpl = env.get_template("slide")?;
    let rendered = tmpl.render(serde_json::Value::Object(context))?;

    Ok(rendered)
}

// ─── Custom minijinja Filters ───────────────────────────────────────

/// wordwrap filter: wrap text to N chars per line, returns joined string.
/// Usage: `{{ slide.body|wordwrap(40) }}`
fn filter_wordwrap(text: String, width: usize) -> String {
    let lines = crate::text::wrap_text(&text, width);
    lines.join("\n")
}

/// b64 filter: convert file path to base64 data URI.
/// Usage: `{{ slide.image|b64 }}`
fn filter_b64(path: String) -> String {
    crate::text::image_to_data_uri(&path).unwrap_or_default()
}

// ─── Template Listing ───────────────────────────────────────────────

/// List all available templates in a directory.
pub fn list_templates(dir: &Path) -> Vec<TemplateDef> {
    let mut templates = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let schema_path = path.join("schema.json");
                if schema_path.exists() {
                    if let Ok(schema_str) = std::fs::read_to_string(&schema_path) {
                        if let Ok(def) = serde_json::from_str::<TemplateDef>(&schema_str) {
                            templates.push(def);
                        }
                    }
                }
            }
        }
    }

    templates.sort_by(|a, b| a.id.cmp(&b.id));
    templates
}

// ─── Validation ─────────────────────────────────────────────────────

/// Validate input data against a template schema.
/// Returns a list of error messages (empty if valid).
pub fn validate_input(template: &TemplateDef, data: &InputData) -> Vec<String> {
    let mut errors = Vec::new();

    // Validate brand fields
    for (name, spec) in &template.brand_fields {
        if spec.required && data.brand.get(name).is_none() {
            errors.push(format!("Missing required brand field: {}", name));
        }
    }

    // Validate slide fields
    for (i, slide) in data.slides.iter().enumerate() {
        for (name, spec) in &template.slide_fields {
            if spec.required && slide.get(name).is_none() {
                errors.push(format!("Slide {}: missing required field: {}", i + 1, name));
            }
            // Check max length for text fields
            if let Some(max_chars) = spec.max {
                if let Some(text) = slide.get(name).and_then(|v| v.as_str()) {
                    if text.chars().count() > max_chars {
                        errors.push(format!(
                            "Slide {}: field '{}' exceeds max length ({} > {})",
                            i + 1,
                            name,
                            text.chars().count(),
                            max_chars
                        ));
                    }
                }
            }
        }
    }

    errors
}
