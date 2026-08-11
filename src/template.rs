//! Template loading, validation, and minijinja processing.

use std::path::{Path, PathBuf};
use crate::schema::{TemplateDef, InputData, FieldType};

/// Load a template by name from ./templates/{name}/ or from a path
pub fn load_template(name: &str) -> anyhow::Result<TemplateDef> {
    let dir = find_template_dir(name)?;
    let schema_path = dir.join("schema.json");
    let schema_str = std::fs::read_to_string(&schema_path)?;
    let def: TemplateDef = serde_json::from_str(&schema_str)?;
    Ok(def)
}

/// Find the template directory
fn find_template_dir(name: &str) -> anyhow::Result<PathBuf> {
    // Check if it's a direct path
    let direct = PathBuf::from(name);
    if direct.is_dir() {
        return Ok(direct);
    }

    // Check ./templates/{name}/
    let template_dir = PathBuf::from("./templates").join(name);
    if template_dir.is_dir() {
        return Ok(template_dir);
    }

    anyhow::bail!("Template '{}' not found. Looked in ./templates/{}/", name, name)
}

/// Load the SVG template string for a given template
pub fn load_svg(name: &str) -> anyhow::Result<String> {
    let dir = find_template_dir(name)?;
    let svg_path = dir.join("template.svg");
    Ok(std::fs::read_to_string(&svg_path)?)
}

/// Apply minijinja token replacement
pub fn apply_template(
    template: &TemplateDef,
    brand: &serde_json::Value,
    slide: &serde_json::Value,
) -> anyhow::Result<String> {
    let svg_template = load_svg(&template.id)?;

    // Build minijinja environment
    let mut env = minijinja::Environment::new();
    env.set_template("slide", &svg_template)?;

    // Build context: merge brand + slide fields
    let mut context = serde_json::Map::new();

    // Add brand fields
    if let serde_json::Value::Object(ref brand_map) = brand {
        for (k, v) in brand_map {
            context.insert(k.clone(), v.clone());
        }
    }

    // Add slide fields
    if let serde_json::Value::Object(ref slide_map) = slide {
        for (k, v) in slide_map {
            context.insert(k.clone(), v.clone());
        }
    }

    // Pre-wrap text fields that need it
    for (field_name, field_spec) in &template.slide_fields {
        if field_spec.field_type == FieldType::Text {
            if let Some(max_chars) = field_spec.max {
                if let Some(text) = slide.get(field_name).and_then(|v| v.as_str()) {
                    let wrapped = crate::text::wrap_text(text, max_chars);
                    context.insert(
                        format!("{}_lines", field_name),
                        serde_json::Value::Array(
                            wrapped.iter()
                                .map(|l| serde_json::Value::String(l.clone()))
                                .collect()
                        ),
                    );
                }
            }
        }
    }

    // Add logo as base64 data URI if present
    if let Some(logo_path) = brand.get("logo").and_then(|v| v.as_str()) {
        if !logo_path.is_empty() {
            let data_uri = crate::text::image_to_data_uri(logo_path)?;
            context.insert("logo_data_uri".to_string(), serde_json::Value::String(data_uri));
        }
    }

    // Render
    let tmpl = env.get_template("slide")?;
    let rendered = tmpl.render(serde_json::Value::Object(context))?;

    Ok(rendered)
}

/// List all available templates in a directory
pub fn list_templates(dir: &Path) -> anyhow::Result<()> {
    println!("Available templates in {}:", dir.display());

    if !dir.is_dir() {
        anyhow::bail!("Directory not found: {}", dir.display());
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let schema_path = path.join("schema.json");
            if schema_path.exists() {
                if let Ok(schema_str) = std::fs::read_to_string(&schema_path) {
                    if let Ok(def) = serde_json::from_str::<TemplateDef>(&schema_str) {
                        println!("  {:20} {} ({}x{})",
                            def.id, def.name, def.dimensions.width, def.dimensions.height);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Validate input data against a template schema
pub fn validate_template(template_name: &str, data_path: &str) -> anyhow::Result<()> {
    let template = load_template(template_name)?;
    let data = InputData::from_file(data_path)?;

    let mut errors = Vec::new();

    // Validate brand fields
    for (name, spec) in &template.brand_fields {
        if spec.required {
            if data.brand.get(name).is_none() {
                errors.push(format!("Missing required brand field: {}", name));
            }
        }
    }

    // Validate slide fields
    for (i, slide) in data.slides.iter().enumerate() {
        for (name, spec) in &template.slide_fields {
            if spec.required {
                if slide.get(name).is_none() {
                    errors.push(format!("Slide {}: missing required field: {}", i + 1, name));
                }
            }
            // Check max length for text fields
            if let Some(max_chars) = spec.max {
                if let Some(text) = slide.get(name).and_then(|v| v.as_str()) {
                    if text.len() > max_chars {
                        errors.push(format!("Slide {}: field '{}' exceeds max length ({} > {})",
                            i + 1, name, text.len(), max_chars));
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        println!("✓ Valid! {} slide(s), all fields OK", data.slides.len());
    } else {
        println!("✗ Validation failed ({} error(s)):", errors.len());
        for e in &errors {
            println!("  - {}", e);
        }
        anyhow::bail!("Validation failed");
    }

    Ok(())
}
