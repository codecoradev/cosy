//! SVG semantic-id tests (#117): field groups survive the usvg write path.

use assert_cmd::Command;
use std::fs;

#[test]
fn og_image_svg_has_semantic_ids() {
    let output = tempfile::NamedTempFile::with_suffix(".svg").unwrap();
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "og-image",
            "-d",
            "templates/og-image/defaults.json",
            "-o",
            output.path().to_str().unwrap(),
            "--format",
            "svg",
        ])
        .assert()
        .success();

    let svg = fs::read_to_string(output.path()).unwrap();
    for id in ["bg", "brand", "f-title", "f-subtitle", "f-author", "f-url"] {
        assert!(
            svg.contains(&format!(r#"id="{id}""#)),
            "og-image SVG must contain id=\"{id}\""
        );
    }
    // Text-to-path still holds alongside the ids.
    assert!(!svg.contains("<text"), "text-to-path must stay on");
}

#[test]
fn code_screenshot_svg_has_semantic_ids() {
    let output = tempfile::NamedTempFile::with_suffix(".svg").unwrap();
    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "code-screenshot",
            "-d",
            "templates/code-screenshot/defaults.json",
            "-o",
            output.path().to_str().unwrap(),
            "--format",
            "svg",
        ])
        .assert()
        .success();

    let svg = fs::read_to_string(output.path()).unwrap();
    for id in ["bg", "brand", "chrome", "f-code"] {
        assert!(
            svg.contains(&format!(r#"id="{id}""#)),
            "code-screenshot SVG must contain id=\"{id}\""
        );
    }
}
