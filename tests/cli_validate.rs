//! Integration tests: CLI validate, templates list, and error handling.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// ─── cosy templates (list) ──────────────────────────────────────────

#[test]
fn test_templates_list_human_readable() {
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["templates", "--dir", "./templates"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available templates"))
        .stdout(predicate::str::contains("stat-card"))
        .stdout(predicate::str::contains("og-image"));
}

#[test]
fn test_templates_list_json_output() {
    let assert = Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["templates", "--dir", "./templates", "--json"])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("templates --json should produce valid JSON");
    assert!(json.is_array(), "Expected array of templates");
    assert!(
        json.as_array().unwrap().len() >= 18,
        "Expected >= 18 templates"
    );
    // Each entry should have id, name, dimensions
    for entry in json.as_array().unwrap() {
        assert!(entry["id"].is_string());
        assert!(entry["name"].is_string());
        assert!(entry["dimensions"]["width"].is_number());
        assert!(entry["dimensions"]["height"].is_number());
    }
}

#[test]
fn test_templates_list_empty_dir() {
    let tmp = tempfile::tempdir().unwrap();
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["templates", "--dir", tmp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("No templates found"));
}

// ─── cosy validate ──────────────────────────────────────────────────

#[test]
fn test_validate_valid_data() {
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "validate",
            "-t",
            "stat-card",
            "-d",
            "templates/stat-card/defaults.json",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Valid!"));
}

#[test]
fn test_validate_missing_required_field() {
    let bad_data = r#"{
        "brand": {"brand_name": "Test"},
        "slides": [{"stat_label": "missing stat_number"}]
    }"#;
    let tmp = tempfile::NamedTempFile::new().unwrap();
    fs::write(tmp.path(), bad_data).unwrap();

    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "validate",
            "-t",
            "stat-card",
            "-d",
            tmp.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("stat_number"));
}

#[test]
fn test_validate_nonexistent_template() {
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "validate",
            "-t",
            "nonexistent",
            "-d",
            "templates/stat-card/defaults.json",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load template"));
}

#[test]
fn test_validate_nonexistent_data_file() {
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "validate",
            "-t",
            "stat-card",
            "-d",
            "/nonexistent/data.json",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Failed to load data"));
}

// ─── cosy render error handling ─────────────────────────────────────

#[test]
fn test_render_nonexistent_template() {
    let output = tempfile::NamedTempFile::new().unwrap();
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "nonexistent-template-xyz",
            "-d",
            "templates/stat-card/defaults.json",
            "-o",
            output.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not found"));
}

#[test]
fn test_render_nonexistent_data_file() {
    let output = tempfile::NamedTempFile::new().unwrap();
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "stat-card",
            "-d",
            "/nonexistent/data.json",
            "-o",
            output.path().to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn test_render_invalid_json_data() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    fs::write(tmp.path(), b"{ not valid json }").unwrap();
    let output = tempfile::NamedTempFile::new().unwrap();

    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "stat-card",
            "-d",
            tmp.path().to_str().unwrap(),
            "-o",
            output.path().to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn test_render_no_input_source() {
    // No --data, --stdin, or --json provided
    let output = tempfile::NamedTempFile::new().unwrap();
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "stat-card",
            "-o",
            output.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn test_render_json_output_on_error() {
    // When --json-output is set, errors should go to stdout as JSON
    let output = tempfile::NamedTempFile::new().unwrap();
    let assert = Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "nonexistent-template",
            "-d",
            "templates/stat-card/defaults.json",
            "-o",
            output.path().to_str().unwrap(),
            "--json-output",
        ])
        .assert()
        .failure()
        .code(2);

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let json: serde_json::Value =
        serde_json::from_str(&stdout).expect("--json-output error should produce valid JSON");
    assert!(json["error"].is_string());
    assert!(json["error"].as_str().unwrap().contains("not found"));
}

// ─── cosy --version ─────────────────────────────────────────────────

#[test]
fn test_version_flag() {
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("cosy"));
}

// ─── cosy --help ────────────────────────────────────────────────────

#[test]
fn test_help_flag() {
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("render"))
        .stdout(predicate::str::contains("templates"))
        .stdout(predicate::str::contains("validate"))
        .stdout(predicate::str::contains("serve"));
}
