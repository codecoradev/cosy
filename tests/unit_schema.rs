//! Unit tests for schema module.
//! Tests TemplateDef parsing, InputData, FieldType enum, and FieldSpec.

use cosy::schema::{Dimensions, FieldSpec, FieldType, InputData, TemplateDef};

// ─── FieldType ──────────────────────────────────────────────────────

#[test]
fn test_field_type_serde_snake_case() {
    // FieldType uses #[serde(rename_all = "snake_case")]
    let json = r#"["text","image","bg","number","color"]"#;
    let types: Vec<FieldType> = serde_json::from_str(json).unwrap();
    assert_eq!(types.len(), 5);
    assert_eq!(types[0], FieldType::Text);
    assert_eq!(types[1], FieldType::Image);
    assert_eq!(types[2], FieldType::Bg);
    assert_eq!(types[3], FieldType::Number);
    assert_eq!(types[4], FieldType::Color);
}

#[test]
fn test_field_type_roundtrip() {
    for ft in [
        FieldType::Text,
        FieldType::Image,
        FieldType::Bg,
        FieldType::Number,
        FieldType::Color,
    ] {
        let json = serde_json::to_string(&ft).unwrap();
        let back: FieldType = serde_json::from_str(&json).unwrap();
        assert_eq!(ft, back);
    }
}

#[test]
fn test_field_type_invalid_value() {
    let result: Result<FieldType, _> = serde_json::from_str("\"invalid\"");
    assert!(result.is_err());
}

// ─── FieldSpec ──────────────────────────────────────────────────────

#[test]
fn test_field_spec_minimal() {
    let json = r#"{"type": "text"}"#;
    let spec: FieldSpec = serde_json::from_str(json).unwrap();
    assert_eq!(spec.field_type, FieldType::Text);
    assert!(!spec.required); // default false
    assert_eq!(spec.max, None);
    assert_eq!(spec.wrap_width, None);
    assert!(spec.options.is_empty());
    assert_eq!(spec.default, None);
}

#[test]
fn test_field_spec_full() {
    let json = r#"{
        "type": "text",
        "required": true,
        "max": 100,
        "wrap_width": 30,
        "options": ["a", "b", "c"],
        "default": "fallback"
    }"#;
    let spec: FieldSpec = serde_json::from_str(json).unwrap();
    assert_eq!(spec.field_type, FieldType::Text);
    assert!(spec.required);
    assert_eq!(spec.max, Some(100));
    assert_eq!(spec.wrap_width, Some(30));
    assert_eq!(spec.options, vec!["a", "b", "c"]);
    assert_eq!(
        spec.default,
        Some(serde_json::Value::String("fallback".into()))
    );
}

// ─── TemplateDef ────────────────────────────────────────────────────

#[test]
fn test_template_def_full_parse() {
    let json = r#"{
        "id": "test-tpl",
        "name": "Test Template",
        "description": "A test",
        "dimensions": {"width": 1080, "height": 1080},
        "fonts": ["Inter", "Space Grotesk"],
        "brand_fields": {
            "brand_name": {"type": "text", "required": true, "max": 30}
        },
        "slide_fields": {
            "title": {"type": "text", "required": true, "max": 60, "wrap_width": 30}
        }
    }"#;
    let def: TemplateDef = serde_json::from_str(json).unwrap();
    assert_eq!(def.id, "test-tpl");
    assert_eq!(def.name, "Test Template");
    assert_eq!(def.dimensions.width, 1080);
    assert_eq!(def.dimensions.height, 1080);
    assert_eq!(def.fonts, vec!["Inter", "Space Grotesk"]);
    assert_eq!(def.brand_fields.len(), 1);
    assert_eq!(def.slide_fields.len(), 1);
}

#[test]
fn test_template_def_defaults_optional_fields() {
    // brand_fields and slide_fields default to empty BTreeMap
    let json = r#"{
        "id": "min",
        "name": "Min",
        "dimensions": {"width": 100, "height": 100}
    }"#;
    let def: TemplateDef = serde_json::from_str(json).unwrap();
    assert!(def.fonts.is_empty());
    assert!(def.brand_fields.is_empty());
    assert!(def.slide_fields.is_empty());
}

#[test]
fn test_template_def_missing_id_fails() {
    let json = r#"{"name": "No ID", "dimensions": {"width": 100, "height": 100}}"#;
    let result: Result<TemplateDef, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

#[test]
fn test_template_def_missing_dimensions_fails() {
    let json = r#"{"id": "x", "name": "No Dims"}"#;
    let result: Result<TemplateDef, _> = serde_json::from_str(json);
    assert!(result.is_err());
}

// ─── Dimensions ─────────────────────────────────────────────────────

#[test]
fn test_dimensions_parse() {
    let json = r#"{"width": 1200, "height": 630}"#;
    let dim: Dimensions = serde_json::from_str(json).unwrap();
    assert_eq!(dim.width, 1200);
    assert_eq!(dim.height, 630);
}

// ─── InputData ──────────────────────────────────────────────────────

#[test]
fn test_input_data_from_json_single_slide() {
    let json = r#"{
        "brand": {"brand_name": "CodeCora"},
        "slides": [{"title": "Hello"}]
    }"#;
    let data = InputData::from_json(json).unwrap();
    assert!(data.is_single_slide());
    assert_eq!(data.slides.len(), 1);
    assert_eq!(
        data.brand.get("brand_name").and_then(|v| v.as_str()),
        Some("CodeCora")
    );
}

#[test]
fn test_input_data_from_json_multi_slide() {
    let json = r#"{
        "brand": {"brand_name": "CodeCora"},
        "slides": [{"title": "Slide 1"}, {"title": "Slide 2"}, {"title": "Slide 3"}]
    }"#;
    let data = InputData::from_json(json).unwrap();
    assert!(!data.is_single_slide());
    assert_eq!(data.slides.len(), 3);
}

#[test]
fn test_input_data_invalid_json() {
    let result = InputData::from_json("{invalid}");
    assert!(result.is_err());
}

#[test]
fn test_input_data_missing_slides() {
    // slides is required — missing should fail
    let json = r#"{"brand": {"brand_name": "Test"}}"#;
    let result = InputData::from_json(json);
    assert!(result.is_err());
}
