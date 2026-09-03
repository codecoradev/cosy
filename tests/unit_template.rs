//! Unit tests for template module.
//! Tests load_template, find_template_dir_for, load_svg, validate_input, list_templates.

use cosy::schema::{FieldType, InputData, TemplateDef};
use cosy::template;
use cosy::text;
use std::path::Path;

// ─── find_template_dir_for ──────────────────────────────────────────

#[test]
fn test_find_template_by_name() {
    let result = template::find_template_dir_for("stat-card");
    assert!(result.is_ok());
    let dir = result.unwrap();
    assert!(dir.is_dir());
    assert!(dir.join("schema.json").exists());
}

#[test]
fn test_find_template_not_found() {
    let result = template::find_template_dir_for("nonexistent-template-xyz");
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not found"));
    assert!(err.contains("nonexistent-template-xyz"));
}

#[test]
fn test_find_template_by_direct_path() {
    // When given a valid directory path, it should return that path directly
    let result = template::find_template_dir_for("./templates/stat-card");
    assert!(result.is_ok());
    let dir = result.unwrap();
    assert!(dir.join("schema.json").exists());
}

// ─── load_template ──────────────────────────────────────────────────

#[test]
fn test_load_template_stat_card() {
    let result = template::load_template("stat-card");
    assert!(result.is_ok());
    let def = result.unwrap();
    assert_eq!(def.id, "stat-card");
    assert!(!def.name.is_empty());
    assert!(def.dimensions.width > 0);
    assert!(def.dimensions.height > 0);
}

#[test]
fn test_load_template_nonexistent() {
    let result = template::load_template("does-not-exist");
    assert!(result.is_err());
}

#[test]
fn test_load_template_has_slide_fields() {
    let def = template::load_template("stat-card").unwrap();
    // stat-card must have slide fields
    assert!(!def.slide_fields.is_empty());
    // Must contain stat_number field
    assert!(def.slide_fields.contains_key("stat_number"));
    let stat_spec = def.slide_fields.get("stat_number").unwrap();
    assert_eq!(stat_spec.field_type, FieldType::Text);
}

// ─── load_svg ───────────────────────────────────────────────────────

#[test]
fn test_load_svg_from_stat_card() {
    let dir = template::find_template_dir_for("stat-card").unwrap();
    let svg = template::load_svg(&dir);
    assert!(svg.is_ok());
    let content = svg.unwrap();
    assert!(content.contains("<svg"));
    assert!(content.contains("xmlns"));
}

#[test]
fn test_load_svg_missing_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let svg = template::load_svg(tmp.path());
    assert!(svg.is_err());
    let err = svg.unwrap_err().to_string();
    assert!(err.contains("template.svg"));
}

// ─── list_templates ─────────────────────────────────────────────────

#[test]
fn test_list_templates_returns_all() {
    let dir = Path::new("./templates");
    let templates = template::list_templates(dir);
    // Must have at least 18 templates
    assert!(
        templates.len() >= 18,
        "Expected >= 18 templates, got {}",
        templates.len()
    );
}

#[test]
fn test_list_templates_sorted_by_id() {
    let dir = Path::new("./templates");
    let templates = template::list_templates(dir);
    let ids: Vec<&str> = templates.iter().map(|t| t.id.as_str()).collect();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted, "Templates should be sorted by id");
}

#[test]
fn test_list_templates_each_has_schema() {
    let dir = Path::new("./templates");
    let templates = template::list_templates(dir);
    for t in &templates {
        assert!(!t.id.is_empty(), "Template id should not be empty");
        assert!(!t.name.is_empty(), "Template name should not be empty");
        assert!(t.dimensions.width > 0, "Width must be positive");
        assert!(t.dimensions.height > 0, "Height must be positive");
    }
}

#[test]
fn test_list_templates_empty_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let templates = template::list_templates(tmp.path());
    assert!(templates.is_empty());
}

#[test]
fn test_list_templates_known_ids() {
    let dir = Path::new("./templates");
    let templates = template::list_templates(dir);
    let ids: Vec<&str> = templates.iter().map(|t| t.id.as_str()).collect();
    // Check some known template IDs exist
    for expected in &[
        "stat-card",
        "twitter-quote",
        "og-image",
        "carousel-default",
        "instagram-story",
    ] {
        assert!(
            ids.contains(expected),
            "Template '{}' missing from list",
            expected
        );
    }
}

// ─── validate_input ─────────────────────────────────────────────────

#[test]
fn test_validate_input_valid_data() {
    let tmpl = template::load_template("stat-card").unwrap();
    let data = InputData::from_json(
        r#"{
            "brand": {"brand_name": "CodeCora"},
            "slides": [{"stat_number": "127%", "stat_label": "Revenue Growth YoY", "source": "Q3 2025"}]
        }"#,
    )
    .unwrap();
    let errors = template::validate_input(&tmpl, &data);
    assert!(
        errors.is_empty(),
        "Expected no validation errors, got: {:?}",
        errors
    );
}

#[test]
fn test_validate_input_missing_required_field() {
    let tmpl = template::load_template("stat-card").unwrap();
    // Missing required stat_number
    let data = InputData::from_json(
        r#"{
            "brand": {"brand_name": "CodeCora"},
            "slides": [{"stat_label": "Some label"}]
        }"#,
    )
    .unwrap();
    let errors = template::validate_input(&tmpl, &data);
    assert!(!errors.is_empty(), "Should have validation errors");
    assert!(
        errors.iter().any(|e| e.contains("stat_number")),
        "Should report missing stat_number"
    );
}

#[test]
fn test_validate_input_max_length_exceeded() {
    // Create a template def with a tight max constraint
    let tmpl_json = r#"{
        "id": "test-max",
        "name": "Test Max",
        "dimensions": {"width": 100, "height": 100},
        "slide_fields": {
            "title": {"type": "text", "required": true, "max": 10}
        }
    }"#;
    let tmpl: TemplateDef = serde_json::from_str(tmpl_json).unwrap();

    // Title exceeds max 10 chars
    let data = InputData::from_json(
        r#"{
            "brand": {},
            "slides": [{"title": "This text is way too long for max 10"}]
        }"#,
    )
    .unwrap();

    let errors = template::validate_input(&tmpl, &data);
    assert!(!errors.is_empty());
    assert!(
        errors
            .iter()
            .any(|e| e.contains("exceeds max length") || e.contains("title")),
        "Should report length violation: {:?}",
        errors
    );
}

#[test]
fn test_validate_input_exact_max_length_ok() {
    let tmpl_json = r#"{
        "id": "test-exact",
        "name": "Test Exact",
        "dimensions": {"width": 100, "height": 100},
        "slide_fields": {
            "title": {"type": "text", "required": true, "max": 5}
        }
    }"#;
    let tmpl: TemplateDef = serde_json::from_str(tmpl_json).unwrap();
    let data = InputData::from_json(r#"{"brand": {}, "slides": [{"title": "hello"}]}"#).unwrap();
    let errors = template::validate_input(&tmpl, &data);
    assert!(
        errors.is_empty(),
        "Exact max length should pass: {:?}",
        errors
    );
}

#[test]
fn test_validate_input_multiple_slides_error_index() {
    let tmpl_json = r#"{
        "id": "test-multi",
        "name": "Test Multi",
        "dimensions": {"width": 100, "height": 100},
        "slide_fields": {
            "title": {"type": "text", "required": true}
        }
    }"#;
    let tmpl: TemplateDef = serde_json::from_str(tmpl_json).unwrap();
    // Slide 2 missing required title
    let data = InputData::from_json(
        r#"{
            "brand": {},
            "slides": [{"title": "OK"}, {}]
        }"#,
    )
    .unwrap();
    let errors = template::validate_input(&tmpl, &data);
    assert!(!errors.is_empty());
    // Error should mention slide 2 (1-indexed)
    assert!(
        errors.iter().any(|e| e.contains("Slide 2")),
        "Should report error on slide 2: {:?}",
        errors
    );
}

// ─── process_template ───────────────────────────────────────────────

#[test]
fn test_process_template_produces_svg() {
    let tmpl = template::load_template("stat-card").unwrap();
    let dir = template::find_template_dir_for("stat-card").unwrap();
    let brand = serde_json::json!({"brand_name": "CodeCora"});
    let slide = serde_json::json!({"stat_number": "42%", "stat_label": "test", "source": "src"});

    let svg =
        template::process_template(&tmpl, &dir, &brand, &slide, text::ImagePolicy::UNRESTRICTED);
    assert!(svg.is_ok());
    let content = svg.unwrap();
    assert!(content.contains("<svg"));
    assert!(content.contains("42%"));
}

// ─── process_template: empty vs non-empty logo/bg_image ─────────────
// These tests kill mutants that delete `!` in the `!path.is_empty()` guards.
// Line 131: `if !logo_path.is_empty()`
// Line 149: `if !bg_path.is_empty()`

#[test]
fn test_process_template_empty_logo_does_not_set_data_uri() {
    // Empty string logo must NOT trigger image_to_data_uri.
    // Mutant flips `!is_empty()` → `is_empty()`, so empty string WOULD trigger it.
    // With empty path, image_to_data_uri returns error, but the guard should prevent
    // even attempting. We verify the SVG still renders correctly.
    let tmpl = template::load_template("stat-card").unwrap();
    let dir = template::find_template_dir_for("stat-card").unwrap();
    let brand = serde_json::json!({
        "brand_name": "Test",
        "logo": ""  // empty string — should be skipped
    });
    let slide = serde_json::json!({"stat_number": "1%", "stat_label": "x", "source": "y"});

    let svg =
        template::process_template(&tmpl, &dir, &brand, &slide, text::ImagePolicy::UNRESTRICTED);
    assert!(svg.is_ok(), "Empty logo should not cause error");
    let content = svg.unwrap();
    // logo_data_uri should NOT appear in output since logo is empty
    assert!(
        !content.contains("logo_data_uri"),
        "Empty logo should not produce logo_data_uri in SVG"
    );
}

#[test]
fn test_process_template_empty_bg_image_does_not_set_data_uri() {
    let tmpl = template::load_template("stat-card").unwrap();
    let dir = template::find_template_dir_for("stat-card").unwrap();
    let brand = serde_json::json!({
        "brand_name": "Test",
        "bg_image": ""  // empty — should be skipped
    });
    let slide = serde_json::json!({"stat_number": "2%", "stat_label": "x", "source": "y"});

    let svg =
        template::process_template(&tmpl, &dir, &brand, &slide, text::ImagePolicy::UNRESTRICTED);
    assert!(svg.is_ok(), "Empty bg_image should not cause error");
    let content = svg.unwrap();
    assert!(
        !content.contains("bg_image_data_uri"),
        "Empty bg_image should not produce bg_image_data_uri in SVG"
    );
}

#[test]
fn test_process_template_non_empty_logo_sets_data_uri() {
    // Create a dummy file in a tempdir to use as "logo".
    // image_to_data_uri reads any file and base64-encodes it — no image validation.
    // This verifies the `!is_empty()` branch IS taken for non-empty paths.
    let tmpdir = tempfile::tempdir().unwrap();
    let logo_path = tmpdir.path().join("logo.png");
    std::fs::write(&logo_path, b"fake-png-data").unwrap();
    let logo_str = logo_path.to_str().unwrap().to_string();

    // First verify image_to_data_uri works with this path
    let data_uri = cosy::text::image_to_data_uri(&logo_str, cosy::text::ImagePolicy::UNRESTRICTED);
    assert!(data_uri.is_ok(), "image_to_data_uri should succeed");
    assert!(data_uri.unwrap().starts_with("data:image/png;base64,"));

    let tmpl = template::load_template("stat-card").unwrap();
    let dir = template::find_template_dir_for("stat-card").unwrap();
    let brand = serde_json::json!({
        "brand_name": "Test",
        "logo": logo_str
    });
    let slide = serde_json::json!({"stat_number": "3%", "stat_label": "x", "source": "y"});

    let svg =
        template::process_template(&tmpl, &dir, &brand, &slide, text::ImagePolicy::UNRESTRICTED);
    assert!(svg.is_ok(), "Valid logo path should not error");
    let content = svg.unwrap();
    // If logo_data_uri is set, template renders <image href="data:image/png;base64,...">.
    // Mutant flips `!is_empty()` → `is_empty()` → skips non-empty path → no data:image.
    assert!(
        content.contains("data:image/png;base64,"),
        "Non-empty logo path should embed data URI in SVG.\n\
         Logo path: {}\n\
         SVG (first 800 chars): {}",
        logo_str,
        &content[..content.len().min(800)]
    );
}
