//! Integration tests: CLI render command.
//! Tests that every template renders successfully from CLI using defaults.json.

use assert_cmd::Command;
use std::fs;
use std::path::Path;

/// Get all template names from the templates directory.
#[allow(dead_code)]
fn all_templates() -> Vec<String> {
    let dir = Path::new("./templates");
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .flatten()
        .filter(|e| {
            e.path().is_dir()
                && e.path().join("schema.json").exists()
                && e.path().join("defaults.json").exists()
        })
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    names.sort();
    names
}

/// Verify a file is a valid PNG by checking magic bytes.
fn assert_valid_png(path: &Path) {
    let bytes = fs::read(path).unwrap_or_else(|_| panic!("Failed to read PNG: {:?}", path));
    assert!(
        bytes.len() > 100,
        "PNG too small ({} bytes): {:?}",
        bytes.len(),
        path
    );
    // PNG magic: 89 50 4E 47 0D 0A 1A 0A
    assert_eq!(
        &bytes[..8],
        b"\x89PNG\r\n\x1a\n",
        "Not a valid PNG: {:?}",
        path
    );
}

// ─── Render each template with defaults.json ────────────────────────

#[test]
fn test_render_stat_card() {
    test_render_template("stat-card");
}

#[test]
fn test_render_twitter_quote() {
    test_render_template("twitter-quote");
}

#[test]
fn test_render_linkedin_card() {
    test_render_template("linkedin-card");
}

#[test]
fn test_render_dev_quote() {
    test_render_template("dev-quote");
}

#[test]
fn test_render_instagram_story() {
    test_render_template("instagram-story");
}

#[test]
fn test_render_youtube_thumb() {
    test_render_template("youtube-thumb");
}

#[test]
fn test_render_github_readme() {
    test_render_template("github-readme");
}

#[test]
fn test_render_tiktok_quote() {
    test_render_template("tiktok-quote");
}

#[test]
fn test_render_newsletter_header() {
    test_render_template("newsletter-header");
}

#[test]
fn test_render_podcast_cover() {
    test_render_template("podcast-cover");
}

#[test]
fn test_render_event_banner() {
    test_render_template("event-banner");
}

#[test]
fn test_render_testimonial() {
    test_render_template("testimonial");
}

#[test]
fn test_render_checklist() {
    test_render_template("checklist");
}

#[test]
fn test_render_comparison() {
    test_render_template("comparison");
}

#[test]
fn test_render_announcement() {
    test_render_template("announcement");
}

#[test]
fn test_render_carousel_default() {
    test_render_template("carousel-default");
}

#[test]
fn test_render_og_image() {
    test_render_template("og-image");
}

#[test]
fn test_render_social_quote() {
    test_render_template("social-quote");
}

/// Helper: render a single template with its defaults.json and verify PNG output.
fn test_render_template(name: &str) {
    // Multi-slide templates need a directory; single-slide need a file.
    // defaults.json determines slide count — read it to decide.
    let defaults_path = format!("templates/{}/defaults.json", name);
    let defaults_str =
        fs::read_to_string(env!("CARGO_MANIFEST_DIR").to_string() + "/" + &defaults_path).unwrap();
    let defaults: serde_json::Value = serde_json::from_str(&defaults_str).unwrap();
    let slide_count = defaults["slides"].as_array().map(|a| a.len()).unwrap_or(1);

    if slide_count > 1 {
        // Multi-slide: output to a directory
        let out_dir = tempfile::tempdir().unwrap();
        let out_path = out_dir.path();

        Command::cargo_bin("cosy")
            .unwrap()
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args([
                "render",
                "-t",
                name,
                "-d",
                &defaults_path,
                "-o",
                out_path.to_str().unwrap(),
                "--scale",
                "1",
            ])
            .assert()
            .success();

        // Verify each slide was generated
        for i in 1..=slide_count {
            let png_path = out_path.join(format!("{:02}.png", i));
            assert!(
                png_path.exists(),
                "Missing slide {} for template {}",
                i,
                name
            );
            assert_valid_png(&png_path);
        }
    } else {
        // Single-slide: output to a file
        let output = tempfile::NamedTempFile::new().unwrap();
        let output_path = output.path();

        Command::cargo_bin("cosy")
            .unwrap()
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args([
                "render",
                "-t",
                name,
                "-d",
                &defaults_path,
                "-o",
                output_path.to_str().unwrap(),
                "--scale",
                "1",
            ])
            .assert()
            .success();

        assert_valid_png(output_path);
    }
}

// ─── Render with scale ──────────────────────────────────────────────

#[test]
fn test_render_scale_2x_larger_than_1x() {
    let out1x = tempfile::NamedTempFile::new().unwrap();
    let out2x = tempfile::NamedTempFile::new().unwrap();

    for (scale, out) in [(1, &out1x), (2, &out2x)] {
        Command::cargo_bin("cosy")
            .unwrap()
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args([
                "render",
                "-t",
                "stat-card",
                "-d",
                "templates/stat-card/defaults.json",
                "-o",
                out.path().to_str().unwrap(),
                "--scale",
                &scale.to_string(),
            ])
            .assert()
            .success();
    }

    let size_1x = fs::metadata(out1x.path()).unwrap().len();
    let size_2x = fs::metadata(out2x.path()).unwrap().len();
    assert!(
        size_2x > size_1x,
        "2x render ({}) should be larger than 1x ({})",
        size_2x,
        size_1x
    );
}

// ─── Render with --json flag (inline data) ──────────────────────────

#[test]
fn test_render_with_json_inline() {
    let json_data = serde_json::json!({
        "brand": {"brand_name": "Test Brand", "brand_handle": "@test"},
        "slides": [{"stat_number": "99%", "stat_label": "test stat", "source": "test source"}]
    })
    .to_string();

    let output = tempfile::NamedTempFile::new().unwrap();

    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "stat-card",
            "--json",
            &json_data,
            "-o",
            output.path().to_str().unwrap(),
            "--scale",
            "1",
        ])
        .assert()
        .success();

    assert_valid_png(output.path());
}

// ─── Render with --json-output flag ─────────────────────────────────

#[test]
fn test_render_json_output_flag() {
    let output = tempfile::NamedTempFile::new().unwrap();

    let assert = Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "stat-card",
            "-d",
            "templates/stat-card/defaults.json",
            "-o",
            output.path().to_str().unwrap(),
            "--scale",
            "1",
            "--json-output",
        ])
        .assert()
        .success();

    // stdout should contain valid JSON with template, files, render_time_ms, slides
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("Failed to parse --json-output: {}\nstdout: {}", e, stdout));
    assert_eq!(json["template"], "stat-card");
    assert_eq!(json["slides"], 1);
    assert!(json["render_time_ms"].as_u64().is_some());
    assert!(json["files"].is_array());
}

// ─── Render with --dump-svg flag ────────────────────────────────────

#[test]
fn test_render_dump_svg() {
    let assert = Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "stat-card",
            "-d",
            "templates/stat-card/defaults.json",
            "-o",
            "/dev/null",
            "--dump-svg",
        ])
        .assert()
        .success();

    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(stdout.contains("<svg"), "dump-svg should output SVG");
    assert!(stdout.contains("xmlns"), "SVG should have xmlns attribute");
}

// ─── Render multi-slide carousel ────────────────────────────────────

#[test]
fn test_render_multi_slide_carousel() {
    let dir = tempfile::tempdir().unwrap();

    let json_data = serde_json::json!({
        "brand": {"brand_name": "CodeCora", "brand_handle": "@codecoradev"},
        "slides": [
            {"title": "Slide 1", "body": "Content 1"},
            {"title": "Slide 2", "body": "Content 2"},
            {"title": "Slide 3", "body": "Content 3"},
        ]
    })
    .to_string();

    let tmp_input = dir.path().join("data.json");
    fs::write(&tmp_input, &json_data).unwrap();

    let out_dir = dir.path().join("output");
    fs::create_dir_all(&out_dir).unwrap();

    Command::cargo_bin("cosy")
        .unwrap()
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "render",
            "-t",
            "carousel-default",
            "-d",
            tmp_input.to_str().unwrap(),
            "-o",
            out_dir.to_str().unwrap(),
            "--scale",
            "1",
        ])
        .assert()
        .success();

    // Should produce 01.png, 02.png, 03.png
    for i in 1..=3 {
        let png_path = out_dir.join(format!("{:02}.png", i));
        assert!(png_path.exists(), "Missing slide {}: {:?}", i, png_path);
        assert_valid_png(&png_path);
    }
}
