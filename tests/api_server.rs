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
    // Find a free port by binding a temporary listener
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener); // free the port for the server to use

    thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(server::run(port, api_key)).unwrap();
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
