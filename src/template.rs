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
    image_policy: crate::text::ImagePolicy,
) -> anyhow::Result<String> {
    let svg_template = load_svg(template_dir)?;

    // Build minijinja environment with custom filters
    let mut env = minijinja::Environment::new();
    env.add_template("slide", &svg_template)?;

    // Register custom filters
    env.add_filter("wordwrap", filter_wordwrap);
    env.add_filter("b64", move |path: String| filter_b64(path, image_policy));

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
            match crate::text::image_to_data_uri(logo_path, image_policy) {
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
            match crate::text::image_to_data_uri_with_size(bg_path, image_policy) {
                Ok(loaded) => {
                    context.insert(
                        "bg_image_data_uri".into(),
                        serde_json::Value::String(loaded.data_uri),
                    );
                    // Positioning geometry: cover-fit at user zoom with a
                    // focal point. Defaults reproduce the old center-crop.
                    let scale = num_field(brand, slide, "bg_image_scale", 1.0, 0.1, 5.0);
                    let fx = num_field(brand, slide, "bg_image_x", 0.5, 0.0, 1.0);
                    let fy = num_field(brand, slide, "bg_image_y", 0.5, 0.0, 1.0);
                    let iw = loaded.width.unwrap_or(template.dimensions.width) as f64;
                    let ih = loaded.height.unwrap_or(template.dimensions.height) as f64;
                    let (gx, gy, gw, gh) = bg_image_geom(
                        template.dimensions.width as f64,
                        template.dimensions.height as f64,
                        iw,
                        ih,
                        scale,
                        fx,
                        fy,
                    );
                    let r2 = |v: f64| (v * 100.0).round() / 100.0;
                    context.insert(
                        "bg_image_geom".into(),
                        serde_json::json!({
                            "x": r2(gx),
                            "y": r2(gy),
                            "w": r2(gw),
                            "h": r2(gh),
                        }),
                    );
                }
                Err(e) => {
                    log::warn!("Failed to load bg_image '{}': {}", bg_path, e);
                }
            }
        }
    }

    // Escape XML-special characters in every context string. Values land in
    // SVG text nodes and attributes; without this, a literal `&` in user data
    // produces malformed XML and the render fails.
    let mut context_value = serde_json::Value::Object(context);
    xml_escape_value(&mut context_value);
    let context = match context_value {
        serde_json::Value::Object(map) => map,
        _ => unreachable!("context was built as an object"),
    };

    // Render
    let tmpl = env.get_template("slide")?;
    let rendered = tmpl.render(serde_json::Value::Object(context))?;

    Ok(rendered)
}

/// Cover-fit geometry for a user-positioned background image.
///
/// Returns `(x, y, w, h)` for the `<image>` element: the image is scaled to
/// cover the canvas (identical to `preserveAspectRatio="slice"`) multiplied
/// by `scale`, then placed so the focal point `(fx, fy)` — fractions of the
/// scaled image — sits at the canvas center. Defaults (`scale=1`, `fx=fy=0.5`)
/// reproduce the old fixed center-crop exactly.
pub fn bg_image_geom(
    canvas_w: f64,
    canvas_h: f64,
    img_w: f64,
    img_h: f64,
    scale: f64,
    fx: f64,
    fy: f64,
) -> (f64, f64, f64, f64) {
    let img_w = img_w.max(1.0);
    let img_h = img_h.max(1.0);
    let cover = (canvas_w / img_w).max(canvas_h / img_h);
    let s = cover * scale.clamp(0.1, 5.0);
    let w = img_w * s;
    let h = img_h * s;
    let x = canvas_w / 2.0 - fx.clamp(0.0, 1.0) * w;
    let y = canvas_h / 2.0 - fy.clamp(0.0, 1.0) * h;
    (x, y, w, h)
}

/// Read a numeric field with brand-over-slide precedence, clamped.
fn num_field(
    brand: &serde_json::Value,
    slide: &serde_json::Value,
    name: &str,
    default: f64,
    min: f64,
    max: f64,
) -> f64 {
    let value = brand
        .get(name)
        .or_else(|| slide.get(name))
        .and_then(|v| v.as_f64())
        .unwrap_or(default);
    value.clamp(min, max)
}

/// Recursively XML-escape all string values in a JSON value.
fn xml_escape_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::String(s) => {
            *s = escape_xml_preserving_entities(s);
        }
        serde_json::Value::Array(items) => {
            for item in items {
                xml_escape_value(item);
            }
        }
        serde_json::Value::Object(map) => {
            for v in map.values_mut() {
                xml_escape_value(v);
            }
        }
        _ => {}
    }
}

/// XML-escape a string while preserving valid entity references.
///
/// The five predefined XML entities (`&amp;` `&lt;` `&gt;` `&quot;` `&apos;`)
/// and numeric references (`&#78;`, `&#x4E;`) pass through untouched, so data
/// that already contains entities is not double-escaped. A bare `&`, an
/// HTML-only entity like `&ldquo;`, or a malformed fragment (`&amp` without
/// the semicolon) is escaped — passing those through would produce malformed
/// XML and fail the render.
fn escape_xml_preserving_entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'&' => {
                let entity = s[i + 1..]
                    .find(';')
                    .map(|end| &s[i..i + end + 2])
                    .filter(|entity| is_valid_entity(&entity[1..entity.len() - 1]));
                if let Some(entity) = entity {
                    out.push_str(entity);
                    i += entity.len();
                } else {
                    out.push_str("&amp;");
                    i += 1;
                }
            }
            b'<' => {
                out.push_str("&lt;");
                i += 1;
            }
            b'>' => {
                out.push_str("&gt;");
                i += 1;
            }
            b'"' => {
                out.push_str("&quot;");
                i += 1;
            }
            b'\'' => {
                out.push_str("&apos;");
                i += 1;
            }
            _ => {
                let ch_len = s[i..].chars().next().map_or(1, char::len_utf8);
                out.push_str(&s[i..i + ch_len]);
                i += ch_len;
            }
        }
    }
    out
}

fn is_valid_entity(name: &str) -> bool {
    const PREDEFINED: [&str; 5] = ["amp", "lt", "gt", "quot", "apos"];
    if let Some(hex) = name.strip_prefix("#x").or_else(|| name.strip_prefix("#X")) {
        return !hex.is_empty() && hex.bytes().all(|b| b.is_ascii_hexdigit());
    }
    if let Some(dec) = name.strip_prefix('#') {
        return !dec.is_empty() && dec.bytes().all(|b| b.is_ascii_digit());
    }
    PREDEFINED.contains(&name)
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
fn filter_b64(path: String, policy: crate::text::ImagePolicy) -> String {
    crate::text::image_to_data_uri(&path, policy).unwrap_or_default()
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
/// Short human-readable JSON value type name for validation messages.
fn type_name(value: &serde_json::Value) -> &'static str {
    match value {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

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
            // Type check: numeric/boolean fields must not be strings.
            // Templates use these fields in arithmetic expressions; a string
            // value fails late inside the Tera engine with a confusing
            // "invalid float literal" error instead of a validation message.
            if let Some(value) = slide.get(name) {
                match spec.field_type {
                    FieldType::Number => {
                        if !value.is_number() {
                            errors.push(format!(
                                "Slide {}: field '{}' must be a number, got {}",
                                i + 1,
                                name,
                                type_name(value)
                            ));
                        }
                    }
                    FieldType::Boolean => {
                        if value.is_boolean() {
                            continue;
                        }
                        errors.push(format!(
                            "Slide {}: field '{}' must be a boolean, got {}",
                            i + 1,
                            name,
                            type_name(value)
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    errors
}

#[cfg(test)]
mod filter_tests {
    use super::*;

    #[test]
    fn test_xml_escape_value_escapes_strings() {
        let mut v =
            serde_json::json!({"a": "x & y < z > w \"q\" 'p'", "b": ["&"], "c": {"d": "<"}});
        xml_escape_value(&mut v);
        assert_eq!(
            v["a"],
            "x &amp; y &lt; z &gt; w &quot;q&quot; &apos;p&apos;"
        );
        assert_eq!(v["b"][0], "&amp;");
        assert_eq!(v["c"]["d"], "&lt;");
    }

    #[test]
    fn test_xml_escape_value_leaves_non_strings() {
        let mut v = serde_json::json!({"n": 42, "f": 1.5, "t": true, "z": null});
        xml_escape_value(&mut v);
        assert_eq!(
            v,
            serde_json::json!({"n": 42, "f": 1.5, "t": true, "z": null})
        );
    }

    #[test]
    fn test_process_template_escapes_ampersand_in_data() {
        let dir = std::path::Path::new("templates/stat-card");
        let template = crate::template::load_template("stat-card").expect("template def");
        let brand = serde_json::json!({});
        let slide =
            serde_json::json!({"stat_number": "R&D & Co", "stat_label": "x", "source": "y"});
        let result = process_template(
            &template,
            dir,
            &brand,
            &slide,
            crate::text::ImagePolicy::SECURE,
        );
        match result {
            Ok(svg) => assert!(
                svg.contains("R&amp;D &amp; Co"),
                "ampersand must be escaped in SVG"
            ),
            Err(_) => panic!("render with '&' in data must not fail"),
        }
    }

    #[test]
    fn test_escape_xml_preserves_predefined_entities() {
        assert_eq!(
            escape_xml_preserving_entities("&amp; &lt; &gt; &quot; &apos;"),
            "&amp; &lt; &gt; &quot; &apos;"
        );
    }

    #[test]
    fn test_escape_xml_preserves_numeric_references() {
        assert_eq!(
            escape_xml_preserving_entities("&#78; &#x4E; &#Xff10;"),
            "&#78; &#x4E; &#Xff10;"
        );
    }

    #[test]
    fn test_escape_xml_escapes_bare_and_unknown() {
        // bare ampersand
        assert_eq!(escape_xml_preserving_entities("R&D"), "R&amp;D");
        // HTML-only entities are invalid XML → escape the ampersand
        assert_eq!(
            escape_xml_preserving_entities("&ldquo;q&rdquo;"),
            "&amp;ldquo;q&amp;rdquo;"
        );
        // malformed: missing semicolon
        assert_eq!(escape_xml_preserving_entities("&amp x"), "&amp;amp x");
        // malformed: empty or non-hex numeric reference
        assert_eq!(escape_xml_preserving_entities("&#;"), "&amp;#;");
        assert_eq!(escape_xml_preserving_entities("&#xZZ;"), "&amp;#xZZ;");
        // ampersand with no semicolon anywhere
        assert_eq!(escape_xml_preserving_entities("a & b"), "a &amp; b");
    }

    #[test]
    fn test_escape_xml_mixed_entities_and_specials() {
        let input = "AT&T &amp; Sons <b> \"q\"";
        let expected = "AT&amp;T &amp; Sons &lt;b&gt; &quot;q&quot;";
        assert_eq!(escape_xml_preserving_entities(input), expected);
    }

    #[test]
    fn test_process_template_preserves_entities_in_render() {
        let dir = std::path::Path::new("templates/stat-card");
        let template = crate::template::load_template("stat-card").expect("template def");
        let brand = serde_json::json!({});
        let slide = serde_json::json!({
            "stat_number": "AT&T &amp; Sons",
            "stat_label": "x",
            "source": "y"
        });
        let result = process_template(
            &template,
            dir,
            &brand,
            &slide,
            crate::text::ImagePolicy::SECURE,
        );
        match result {
            Ok(svg) => {
                assert!(
                    svg.contains("AT&amp;T &amp; Sons"),
                    "bare & escaped but existing entity preserved, got: {svg}"
                );
            }
            Err(_) => panic!("render with pre-existing entity must not fail"),
        }
    }

    #[test]
    fn test_filter_wordwrap_returns_wrapped_text() {
        let result = filter_wordwrap("Hello World Foo Bar".into(), 5);
        assert!(
            result.contains('\n'),
            "wordwrap should produce newlines, got: {:?}",
            result
        );
        assert!(
            !result.is_empty(),
            "wordwrap should not return empty string"
        );
        assert!(
            result.lines().all(|l| l.len() <= 5),
            "each line should be within width"
        );
    }

    #[test]
    fn test_filter_wordwrap_preserves_content() {
        let result = filter_wordwrap("CodeCora".into(), 100);
        assert_eq!(result, "CodeCora");
    }

    #[test]
    fn test_filter_b64_returns_data_uri() {
        let filepath = std::env::temp_dir().join("cosy_filter_test.png");
        std::fs::write(&filepath, b"fake-image").unwrap();
        let path_str = filepath.to_str().unwrap().to_string();

        let result = filter_b64(
            path_str,
            crate::text::ImagePolicy {
                allow_private: true,
                allow_local: true,
            },
        );
        assert!(
            result.starts_with("data:image/png;base64,"),
            "b64 filter should return data URI, got: {}",
            &result[..result.len().min(50)]
        );
        assert!(!result.is_empty());

        let _ = std::fs::remove_file(&filepath);
    }

    #[test]
    fn test_filter_b64_invalid_path_returns_empty() {
        let result = filter_b64(
            "/nonexistent/path/to/file.png".into(),
            crate::text::ImagePolicy::UNRESTRICTED,
        );
        assert_eq!(result, "");
    }
}
