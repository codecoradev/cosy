//! Integration tests: HTTP API server.
//!
//! Each test starts the server on a unique port, makes HTTP requests,
//! and validates responses. Uses reqwest (rustls) as the HTTP client.

use cosy::server;
use reqwest::blocking::Client;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;

/// Start the API server on an available port in a background thread.
/// Auth disabled (no API key) — for testing endpoint behavior.
/// Returns the base URL (e.g. "http://127.0.0.1:XXXXX").
fn start_server() -> String {
    start_server_with_key(None)
}

/// Start the API server with an optional API key for auth testing.
fn start_server_with_key(api_key: Option<String>) -> String {
    start_server_with_keys(api_key, None)
}

/// Start the API server with an API key and an optional separate signing key.
fn start_server_with_keys(api_key: Option<String>, signing_key: Option<String>) -> String {
    // Find a free port by binding a temporary listener
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener); // free the port for the server to use

    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime
            .block_on(server::run_with(
                "127.0.0.1",
                port,
                api_key,
                signing_key,
                cosy::text::ImagePolicy::UNRESTRICTED,
            ))
            .unwrap();
    });

    // Wait for server to be ready (poll health endpoint)
    let url = format!("http://127.0.0.1:{}", port);
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();
    for _ in 0..50 {
        if client.get(format!("{}/api/health", url)).send().is_ok() {
            return url;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    panic!("Server failed to start on port {}", port);
}

/// Get a reqwest blocking client with reasonable timeout.
fn http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
}

// ─── GET /api/health ────────────────────────────────────────────────

#[test]
fn test_health_returns_ok() {
    let url = start_server();
    let resp = http_client()
        .get(format!("{}/api/health", url))
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
}

#[test]
fn test_health_response_body() {
    let url = start_server();
    let resp = http_client()
        .get(format!("{}/api/health", url))
        .send()
        .unwrap();
    let json: serde_json::Value = resp.json().unwrap();
    assert_eq!(json["status"], "ok");
    assert!(
        json["version"].as_str().is_some(),
        "version should be a string"
    );
    assert!(
        json["templates"].as_u64().unwrap() >= 18,
        "Expected >= 18 templates"
    );
    assert_eq!(
        json["auth_enabled"], false,
        "auth should be disabled when no key set"
    );
}

#[test]
fn test_health_shows_auth_enabled() {
    let url = start_server_with_key(Some("secret123".into()));
    let resp = http_client()
        .get(format!("{}/api/health", url))
        .send()
        .unwrap();
    let json: serde_json::Value = resp.json().unwrap();
    assert_eq!(json["auth_enabled"], true);
}

// ─── GET /api/templates (no auth) ───────────────────────────────────

#[test]
fn test_templates_returns_list() {
    let url = start_server();
    let resp = http_client()
        .get(format!("{}/api/templates", url))
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().unwrap();
    assert!(json.is_array(), "Expected array");
    assert!(
        json.as_array().unwrap().len() >= 18,
        "Expected >= 18 templates"
    );
}

#[test]
fn test_templates_contains_stat_card() {
    let url = start_server();
    let resp = http_client()
        .get(format!("{}/api/templates", url))
        .send()
        .unwrap();
    let json: serde_json::Value = resp.json().unwrap();
    let arr = json.as_array().unwrap();
    let has_stat_card = arr.iter().any(|t| t["id"].as_str() == Some("stat-card"));
    assert!(has_stat_card, "Template list should contain stat-card");
}

#[test]
fn test_templates_each_has_dimensions() {
    let url = start_server();
    let resp = http_client()
        .get(format!("{}/api/templates", url))
        .send()
        .unwrap();
    let json: serde_json::Value = resp.json().unwrap();
    for entry in json.as_array().unwrap() {
        assert!(entry["dimensions"]["width"].as_u64().unwrap() > 0);
        assert!(entry["dimensions"]["height"].as_u64().unwrap() > 0);
    }
}

// ─── POST /api/render (no auth) ─────────────────────────────────────

#[test]
fn test_render_returns_png() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "stat-card",
        "scale": 1.0,
        "data": {
            "brand": {"brand_name": "CodeCora", "brand_handle": "@codecoradev"},
            "slides": [{"stat_number": "127%", "stat_label": "Revenue Growth YoY", "source": "Q3 2025 Report"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers().get("content-type").unwrap(), "image/png");

    let bytes = resp.bytes().unwrap();
    assert!(
        bytes.len() > 1000,
        "PNG should be at least 1KB, got {} bytes",
        bytes.len()
    );
    // Verify PNG magic bytes
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
}

#[test]
fn test_render_different_template() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "og-image",
        "scale": 1.0,
        "data": {
            "brand": {"brand_name": "Test"},
            "slides": [{"title": "OG Image Test", "subtitle": "A test"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    let bytes = resp.bytes().unwrap();
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
}

#[test]
fn test_render_default_scale() {
    let url = start_server();
    // No scale field — should default to 2.0
    let body = serde_json::json!({
        "template": "stat-card",
        "data": {
            "brand": {"brand_name": "Scale Test"},
            "slides": [{"stat_number": "50%", "stat_label": "default scale", "source": "test"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    let bytes = resp.bytes().unwrap();
    assert!(bytes.len() > 5000, "2x render should be substantial");
}

#[test]
fn test_render_json_multi_slide() {
    let url = start_server();
    // response_format=json renders ALL slides and returns a JSON envelope
    let body = serde_json::json!({
        "template": "carousel-default",
        "response_format": "json",
        "scale": 0.5,
        "data": {
            "brand": {"brand_name": "Multi Test"},
            "slides": [
                {"eyebrow": "s1", "headline": "Slide One", "body": "first"},
                {"eyebrow": "s2", "headline": "Slide Two", "body": "second"},
                {"eyebrow": "s3", "headline": "Slide Three", "body": "third"}
            ]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().unwrap();
    assert_eq!(json["template"], "carousel-default");
    assert_eq!(json["slides"], 3);
    assert_eq!(json["data"].as_array().unwrap().len(), 3);
    for (i, slide) in json["data"].as_array().unwrap().iter().enumerate() {
        assert_eq!(slide["index"], i);
        let b64 = slide["png_base64"].as_str().unwrap();
        assert!(b64.len() > 1000, "slide {} png should be substantial", i);
    }
    // Slide dimensions: 1080x1350 at 0.5 scale = 540x675
    assert_eq!(json["width"], 540);
    assert_eq!(json["height"], 675);
}

#[test]
fn test_render_slide_index_png() {
    let url = start_server();
    // slide_index picks a specific slide with png format (default)
    let body = serde_json::json!({
        "template": "carousel-default",
        "slide_index": 1,
        "scale": 0.5,
        "data": {
            "brand": {"brand_name": "Index Test"},
            "slides": [
                {"eyebrow": "s1", "headline": "Slide One", "body": "first"},
                {"eyebrow": "s2", "headline": "Slide Two", "body": "second"}
            ]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "image/png");
    let bytes = resp.bytes().unwrap();
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
}

#[test]
fn test_render_slide_index_out_of_range() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "carousel-default",
        "slide_index": 5,
        "scale": 0.5,
        "data": {
            "brand": {"brand_name": "Range Test"},
            "slides": [
                {"eyebrow": "s1", "headline": "Only", "body": "one"}
            ]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 400);
    let json: serde_json::Value = resp.json().unwrap();
    assert!(json["error"].as_str().unwrap().contains("out of range"));
}

#[test]
fn test_render_empty_slides_rejected() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "carousel-default",
        "data": {"brand": {"brand_name": "Empty"}, "slides": []}
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 400);
    let json: serde_json::Value = resp.json().unwrap();
    assert!(json["error"]
        .as_str()
        .unwrap()
        .contains("at least one slide"));
}

#[test]
fn test_render_nonexistent_template() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "nonexistent-xyz",
        "data": {
            "brand": {},
            "slides": [{}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert!(resp.status().is_client_error());
    let json: serde_json::Value = resp.json().unwrap();
    assert!(json["error"].as_str().unwrap().contains("nonexistent-xyz"));
}

#[test]
fn test_render_malformed_json_body() {
    let url = start_server();
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .header("content-type", "application/json")
        .body("{invalid json}")
        .send()
        .unwrap();
    // axum returns 400 for unparseable JSON
    assert!(resp.status().is_client_error());
}

#[test]
fn test_render_missing_data_field() {
    let url = start_server();
    // Missing required "data" field
    let body = serde_json::json!({
        "template": "stat-card"
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert!(resp.status().is_client_error());
}

#[test]
fn test_render_missing_template_field() {
    let url = start_server();
    let body = serde_json::json!({
        "data": {"brand": {}, "slides": [{}]}
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert!(resp.status().is_client_error());
}

// ─── Auth tests ─────────────────────────────────────────────────────

#[test]
fn test_auth_health_public_with_key() {
    // Health should always be accessible, even with auth enabled
    let url = start_server_with_key(Some("mysecret".into()));
    let resp = http_client()
        .get(format!("{}/api/health", url))
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
}

#[test]
fn test_auth_templates_rejected_without_token() {
    let url = start_server_with_key(Some("mysecret".into()));
    let resp = http_client()
        .get(format!("{}/api/templates", url))
        .send()
        .unwrap();
    assert_eq!(resp.status(), 401);
}

#[test]
fn test_auth_templates_rejected_wrong_token() {
    let url = start_server_with_key(Some("mysecret".into()));
    let resp = http_client()
        .get(format!("{}/api/templates", url))
        .header("Authorization", "Bearer wrongtoken")
        .send()
        .unwrap();
    assert_eq!(resp.status(), 401);
}

#[test]
fn test_auth_templates_accepted_correct_token() {
    let url = start_server_with_key(Some("mysecret".into()));
    let resp = http_client()
        .get(format!("{}/api/templates", url))
        .header("Authorization", "Bearer mysecret")
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().unwrap();
    assert!(json.as_array().unwrap().len() >= 18);
}

#[test]
fn test_auth_render_rejected_without_token() {
    let url = start_server_with_key(Some("mysecret".into()));
    let body = serde_json::json!({
        "template": "stat-card",
        "scale": 1.0,
        "data": {
            "brand": {"brand_name": "Test"},
            "slides": [{"stat_number": "50%", "stat_label": "test", "source": "x"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 401);
}

#[test]
fn test_auth_render_accepted_correct_token() {
    let url = start_server_with_key(Some("mysecret".into()));
    let body = serde_json::json!({
        "template": "stat-card",
        "scale": 1.0,
        "data": {
            "brand": {"brand_name": "Test"},
            "slides": [{"stat_number": "50%", "stat_label": "test", "source": "x"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .header("Authorization", "Bearer mysecret")
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers().get("content-type").unwrap(), "image/png");
}

// ─── Unknown routes ─────────────────────────────────────────────────

#[test]
fn test_unknown_route_404() {
    let url = start_server();
    let resp = http_client()
        .get(format!("{}/api/nonexistent", url))
        .send()
        .unwrap();
    assert_eq!(resp.status(), 404);
}

#[test]
fn test_root_route_404() {
    let url = start_server();
    let resp = http_client().get(&url).send().unwrap();
    // No route at "/" — should be 404
    assert_eq!(resp.status(), 404);
}

// ─── CORS headers ───────────────────────────────────────────────────

#[test]
fn test_cors_header_present() {
    let url = start_server();
    let resp = http_client()
        .get(format!("{}/api/health", url))
        .header("Origin", "https://example.com")
        .send()
        .unwrap();
    // tower-http CorsLayer should add access-control-allow-origin
    let cors = resp.headers().get("access-control-allow-origin");
    assert!(cors.is_some(), "CORS header should be present");
}

// ─── Image format: WebP (image_format field) ─────────────────────────

/// Verify WebP bytes: RIFF container + WEBP fourcc.
fn assert_webp_bytes(bytes: &[u8], label: &str) {
    assert!(
        bytes.len() > 32,
        "{} too small ({} bytes)",
        label,
        bytes.len()
    );
    assert_eq!(&bytes[..4], b"RIFF", "{} not a RIFF container", label);
    assert_eq!(&bytes[8..12], b"WEBP", "{} not a WebP payload", label);
}

#[test]
fn test_render_webp_binary_response() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "stat-card",
        "image_format": "webp",
        "scale": 1.0,
        "data": {
            "brand": {"brand_name": "WebP Test"},
            "slides": [{"stat_number": "42%", "stat_label": "webp binary", "source": "test"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "image/webp");
    assert_webp_bytes(&resp.bytes().unwrap(), "webp binary response");
}

#[test]
fn test_render_svg_binary_response() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "stat-card",
        "image_format": "svg",
        "scale": 1.0,
        "data": {
            "brand": {"brand_name": "SVG Test"},
            "slides": [{"stat_number": "31%", "stat_label": "svg binary", "source": "test"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "image/svg+xml");
    let svg = resp.text().unwrap();
    assert!(
        svg.starts_with("<?xml") || svg.starts_with("<svg"),
        "not an SVG document"
    );
    assert!(
        !svg.contains("<text"),
        "raw <text> found — text-to-path failed"
    );
    assert!(!svg.contains("font-family"), "font-family attribute leaked");
}

#[test]
fn test_render_svg_json_envelope() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "carousel-default",
        "response_format": "json",
        "image_format": "svg",
        "scale": 0.5,
        "data": {
            "brand": {"brand_name": "SVG JSON"},
            "slides": [
                {"eyebrow": "s1", "headline": "Slide One", "body": "first"},
                {"eyebrow": "s2", "headline": "Slide Two", "body": "second"}
            ]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().unwrap();
    assert_eq!(json["slides"], 2);
    for slide in json["data"].as_array().unwrap() {
        assert_eq!(slide["image_format"], "svg");
        let b64 = slide["png_base64"].as_str().unwrap();
        use base64::Engine;
        let raw = base64::engine::general_purpose::STANDARD
            .decode(b64)
            .unwrap();
        let svg = String::from_utf8(raw).unwrap();
        assert!(svg.starts_with("<?xml") || svg.starts_with("<svg"));
        assert!(!svg.contains("<text"), "raw <text> found in envelope slide");
    }
}

#[test]
fn test_render_webp_json_envelope() {
    let url = start_server();
    // image_format is orthogonal to response_format: JSON envelope can
    // carry WebP entries.
    let body = serde_json::json!({
        "template": "carousel-default",
        "response_format": "json",
        "image_format": "webp",
        "scale": 0.5,
        "data": {
            "brand": {"brand_name": "WebP JSON"},
            "slides": [
                {"eyebrow": "s1", "headline": "Slide One", "body": "first"},
                {"eyebrow": "s2", "headline": "Slide Two", "body": "second"}
            ]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().unwrap();
    assert_eq!(json["slides"], 2);
    for slide in json["data"].as_array().unwrap() {
        assert_eq!(slide["image_format"], "webp");
        let b64 = slide["png_base64"].as_str().unwrap();
        use base64::Engine;
        let raw = base64::engine::general_purpose::STANDARD
            .decode(b64)
            .unwrap();
        assert_webp_bytes(&raw, "json envelope webp slide");
    }
}

#[test]
fn test_render_unknown_image_format_400() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "stat-card",
        "image_format": "avif",
        "scale": 1.0,
        "data": {
            "brand": {"brand_name": "Bad Format"},
            "slides": [{"stat_number": "1%", "stat_label": "x", "source": "x"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 422);
}

// ─── Metadata passthrough ────────────────────────────────────────────

#[test]
fn test_metadata_echoed_in_json_envelope() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "carousel-default",
        "response_format": "json",
        "scale": 0.5,
        "metadata": {"job_id": "render-42", "source": "blog-engine"},
        "data": {
            "brand": {"brand_name": "Meta Test"},
            "slides": [
                {"eyebrow": "s1", "headline": "Slide One", "body": "first"},
                {"eyebrow": "s2", "headline": "Slide Two", "body": "second"}
            ]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().unwrap();
    assert_eq!(json["metadata"]["job_id"], "render-42");
    assert_eq!(json["metadata"]["source"], "blog-engine");
}

#[test]
fn test_metadata_absent_by_default() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "carousel-default",
        "response_format": "json",
        "scale": 0.5,
        "data": {
            "brand": {"brand_name": "No Meta"},
            "slides": [{"eyebrow": "s1", "headline": "One", "body": "x"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    let json: serde_json::Value = resp.json().unwrap();
    assert!(
        json.get("metadata").is_none(),
        "metadata must be absent when not provided"
    );
}

#[test]
fn test_metadata_too_large_413() {
    let url = start_server();
    let big_string = "x".repeat(5000);
    let body = serde_json::json!({
        "template": "stat-card",
        "scale": 0.5,
        "metadata": big_string,
        "data": {
            "brand": {"brand_name": "Big Meta"},
            "slides": [{"stat_number": "1%", "stat_label": "x", "source": "x"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 413);
}

#[test]
fn test_metadata_binary_response_still_ok() {
    // Binary responses can't carry metadata in the body, but providing it
    // must not fail the render.
    let url = start_server();
    let body = serde_json::json!({
        "template": "stat-card",
        "scale": 0.5,
        "metadata": {"note": "binary mode"},
        "data": {
            "brand": {"brand_name": "Bin Meta"},
            "slides": [{"stat_number": "5%", "stat_label": "x", "source": "x"}]
        }
    });
    let resp = http_client()
        .post(format!("{}/api/render", url))
        .json(&body)
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "image/png");
}

// ─── GET /r/:template — signed render URLs ───────────────────────────

fn make_signed_url(base: &str, key: &str, template: &str, data_json: &str, ext: &str) -> String {
    server::signed_get_url(base, key, template, data_json, ext)
}

#[test]
fn test_signed_get_renders_png() {
    let key = "signkey-1".to_string();
    let url = start_server_with_key(Some(key.clone()));
    let data = r#"{"brand":{"brand_name":"OG Test"},"slides":[{"stat_number":"88%","stat_label":"og image","source":"blog"}]}"#;
    let signed = make_signed_url(&url, &key, "stat-card", data, "png");

    let resp = http_client().get(&signed).send().unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "image/png");
    assert_eq!(resp.headers()["cache-control"], "public, max-age=3600");
    let bytes = resp.bytes().unwrap();
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
}

#[test]
fn test_signed_get_webp_extension() {
    let key = "signkey-2".to_string();
    let url = start_server_with_key(Some(key.clone()));
    let data = r#"{"brand":{"brand_name":"OG Test"},"slides":[{"stat_number":"7%","stat_label":"webp og","source":"blog"}]}"#;
    let signed = make_signed_url(&url, &key, "stat-card", data, "webp");

    let resp = http_client().get(&signed).send().unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers()["content-type"], "image/webp");
    assert_webp_bytes(&resp.bytes().unwrap(), "signed GET webp");
}

#[test]
fn test_signed_get_invalid_signature_403() {
    let key = "signkey-3".to_string();
    let url = start_server_with_key(Some(key.clone()));
    let signed = make_signed_url(
        &url,
        &key,
        "stat-card",
        r#"{"brand":{},"slides":[]}"#,
        "png",
    );
    let tampered = signed.replace(&signed[signed.find("sig=").unwrap() + 4..], &"0".repeat(64));

    let resp = http_client().get(&tampered).send().unwrap();
    assert_eq!(resp.status(), 403);
}

#[test]
fn test_signed_get_signature_bound_to_template() {
    // A valid signature for template A must not authorize template B.
    let key = "signkey-4".to_string();
    let url = start_server_with_key(Some(key.clone()));
    let data = r#"{"brand":{"brand_name":"X"},"slides":[{"stat_number":"1%","stat_label":"x","source":"x"}]}"#;
    let signed_for_stat = make_signed_url(&url, &key, "stat-card", data, "png");
    // Transplant d+sig onto a different template path.
    let transplanted = signed_for_stat.replacen("stat-card", "og-image", 1);

    let resp = http_client().get(&transplanted).send().unwrap();
    assert_eq!(resp.status(), 403);
}

#[test]
fn test_signed_get_disabled_without_key() {
    // No API key → dev mode → signed route must be disabled (404).
    let url = start_server();
    let signed = make_signed_url(
        &url,
        "anykey",
        "stat-card",
        r#"{"brand":{},"slides":[]}"#,
        "png",
    );
    let resp = http_client().get(&signed).send().unwrap();
    assert_eq!(resp.status(), 404);
}

#[test]
fn test_signed_get_unknown_template_400() {
    let key = "signkey-5".to_string();
    let url = start_server_with_key(Some(key.clone()));
    let data = r#"{"brand":{},"slides":[]}"#;
    let signed = make_signed_url(&url, &key, "no-such-template", data, "png");
    let resp = http_client().get(&signed).send().unwrap();
    assert_eq!(resp.status(), 400);
}

#[test]
fn test_signed_get_bad_extension_400() {
    let key = "signkey-6".to_string();
    let url = start_server_with_key(Some(key.clone()));
    let signed = make_signed_url(
        &url,
        &key,
        "stat-card",
        r#"{"brand":{},"slides":[]}"#,
        "jpg",
    );
    let resp = http_client().get(&signed).send().unwrap();
    assert_eq!(resp.status(), 400);
}

// ─── Hardening (#127) ───────────────────────────────────────────────

fn render_body(extra: serde_json::Value) -> serde_json::Value {
    let mut body = serde_json::json!({
        "template": "stat-card",
        "scale": 0.5,
        "data": {
            "brand": {"brand_name": "T"},
            "slides": [{"stat_number": "1%", "stat_label": "x", "source": "x"}]
        }
    });
    for (k, v) in extra.as_object().unwrap() {
        body[k] = v.clone();
    }
    body
}

fn post_render(url: &str, body: &serde_json::Value) -> reqwest::blocking::Response {
    http_client()
        .post(format!("{}/api/render", url))
        .json(body)
        .send()
        .unwrap()
}

#[test]
fn test_render_scale_out_of_range_rejected() {
    let url = start_server();
    for scale in [0.0, -1.0, 0.01, 4.5, 12.0, 1e9] {
        let resp = post_render(&url, &render_body(serde_json::json!({ "scale": scale })));
        assert_eq!(resp.status(), 400, "scale {scale} must be rejected");
    }
    // 4.01 is just over the cap; the in-range path is covered by every
    // other render test (a 4.0 render is too slow for a debug-build test).
    let over = post_render(&url, &render_body(serde_json::json!({ "scale": 4.01 })));
    assert_eq!(over.status(), 400);
}

#[test]
fn test_render_path_template_rejected() {
    let url = start_server();
    for name in [
        "./templates/stat-card",
        "templates/stat-card",
        "../cosy/templates/stat-card",
        "/tmp",
        ".",
        "",
    ] {
        let resp = post_render(&url, &render_body(serde_json::json!({ "template": name })));
        assert_eq!(resp.status(), 400, "template {name:?} must be rejected");
    }
}

#[test]
fn test_render_validates_input() {
    let url = start_server();
    let body = serde_json::json!({
        "template": "stat-card",
        "scale": 0.5,
        "data": {"brand": {}, "slides": [{}]}
    });
    let resp = post_render(&url, &body);
    assert_eq!(resp.status(), 400);
    let json: serde_json::Value = resp.json().unwrap();
    assert!(json["error"]
        .as_str()
        .unwrap()
        .contains("validation failed"));
}

#[test]
fn test_render_json_too_many_slides_413() {
    let url = start_server();
    let slide = serde_json::json!({"stat_number": "1%", "stat_label": "x", "source": "x"});
    let slides: Vec<_> = (0..server::MAX_SLIDES_PER_REQUEST + 1)
        .map(|_| slide.clone())
        .collect();
    let body = serde_json::json!({
        "template": "stat-card",
        "scale": 0.25,
        "response_format": "json",
        "data": {"brand": {"brand_name": "T"}, "slides": slides}
    });
    assert_eq!(post_render(&url, &body).status(), 413);
}

#[test]
fn test_empty_api_key_means_auth_disabled() {
    // docker-compose passes COSY_API_KEY="" when unset
    let url = start_server_with_key(Some(String::new()));
    let resp = http_client()
        .get(format!("{}/api/templates", url))
        .send()
        .unwrap();
    assert_eq!(resp.status(), 200);
    let health: serde_json::Value = http_client()
        .get(format!("{}/api/health", url))
        .send()
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(health["auth_enabled"], false);
}

// ─── Signing key + expiry (#129) ────────────────────────────────────

const SIGNED_DATA: &str =
    r#"{"brand":{"brand_name":"T"},"slides":[{"stat_number":"1%","stat_label":"x","source":"x"}]}"#;

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[test]
fn test_signed_get_separate_signing_key() {
    let url = start_server_with_keys(Some("api-key".into()), Some("sign-key".into()));
    // API key must NOT be able to sign when a dedicated signing key is set
    let with_api = server::signed_get_url(&url, "api-key", "stat-card", SIGNED_DATA, "png");
    assert_eq!(http_client().get(&with_api).send().unwrap().status(), 403);
    let with_sign = server::signed_get_url(&url, "sign-key", "stat-card", SIGNED_DATA, "png");
    assert_eq!(http_client().get(&with_sign).send().unwrap().status(), 200);
}

#[test]
fn test_signed_get_enabled_by_signing_key_alone() {
    let url = start_server_with_keys(None, Some("sign-key".into()));
    let signed = server::signed_get_url(&url, "sign-key", "stat-card", SIGNED_DATA, "png");
    assert_eq!(http_client().get(&signed).send().unwrap().status(), 200);
}

#[test]
fn test_signed_get_expiry() {
    let key = "exp-key";
    let url = start_server_with_key(Some(key.into()));
    let exp = now_secs() + 120;
    let ok =
        server::signed_get_url_with_expiry(&url, key, "stat-card", SIGNED_DATA, "png", Some(exp));
    let resp = http_client().get(&ok).send().unwrap();
    assert_eq!(resp.status(), 200);
    let cc = resp.headers()["cache-control"]
        .to_str()
        .unwrap()
        .to_string();
    let max_age: u64 = cc.trim_start_matches("public, max-age=").parse().unwrap();
    assert!(max_age <= 120, "cache must not outlive exp, got {cc}");

    let expired = server::signed_get_url_with_expiry(
        &url,
        key,
        "stat-card",
        SIGNED_DATA,
        "png",
        Some(now_secs() - 5),
    );
    assert_eq!(http_client().get(&expired).send().unwrap().status(), 403);
}

#[test]
fn test_signed_get_exp_cannot_be_stripped_or_added() {
    let key = "exp-key-2";
    let url = start_server_with_key(Some(key.into()));
    // strip exp from an expiring URL (even an already-expired one)
    let expired = server::signed_get_url_with_expiry(
        &url,
        key,
        "stat-card",
        SIGNED_DATA,
        "png",
        Some(now_secs() - 5),
    );
    let stripped = expired.split("&exp=").next().unwrap().to_string()
        + "&sig="
        + expired.split("&sig=").nth(1).unwrap();
    assert_eq!(http_client().get(&stripped).send().unwrap().status(), 403);
    // add exp to a legacy (no-exp) signature
    let legacy = server::signed_get_url(&url, key, "stat-card", SIGNED_DATA, "png");
    let injected = format!("{legacy}&exp={}", now_secs() + 3600);
    assert_eq!(http_client().get(&injected).send().unwrap().status(), 403);
}

// ─── Cora scan findings (#136) ──────────────────────────────────────

#[test]
fn test_bearer_scheme_is_case_insensitive() {
    let url = start_server_with_key(Some("k".into()));
    for scheme in ["Bearer", "bearer", "BEARER"] {
        let resp = http_client()
            .get(format!("{}/api/templates", url))
            .header("Authorization", format!("{scheme} k"))
            .send()
            .unwrap();
        assert_eq!(resp.status(), 200, "scheme {scheme}");
    }
    let basic = http_client()
        .get(format!("{}/api/templates", url))
        .header("Authorization", "Basic k")
        .send()
        .unwrap();
    assert_eq!(basic.status(), 401);
}

#[test]
fn test_json_envelope_dimensions_match_image() {
    use base64::Engine;
    let url = start_server();
    // 0.3333 × canvas width has a fractional part ≥ .5 → truncation ≠ rounding
    let body = render_body(serde_json::json!({ "scale": 0.3333, "response_format": "json" }));
    let json: serde_json::Value = post_render(&url, &body).json().unwrap();
    let png = base64::engine::general_purpose::STANDARD
        .decode(json["data"][0]["png_base64"].as_str().unwrap())
        .unwrap();
    let w = u32::from_be_bytes(png[16..20].try_into().unwrap());
    let h = u32::from_be_bytes(png[20..24].try_into().unwrap());
    assert_eq!(json["width"], w);
    assert_eq!(json["height"], h);
}
