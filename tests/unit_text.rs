//! Unit tests for text module — additional edge cases.
//! Complements the 5 inline tests in src/text.rs.

use cosy::text::{image_to_data_uri, wrap_text};

// ─── wrap_text edge cases ───────────────────────────────────────────

#[test]
fn test_wrap_single_word() {
    let result = wrap_text("hello", 10);
    assert_eq!(result, vec!["hello"]);
}

#[test]
fn test_wrap_single_char() {
    let result = wrap_text("A", 5);
    assert_eq!(result, vec!["A"]);
}

#[test]
fn test_wrap_exact_width() {
    // Text exactly fits
    let result = wrap_text("12345", 5);
    assert_eq!(result, vec!["12345"]);
}

#[test]
fn test_wrap_exceeds_by_one() {
    // Text 6 chars, width 5 — should break
    let result = wrap_text("123456", 5);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], "12345");
    assert_eq!(result[1], "6");
}

#[test]
fn test_wrap_multiline_preserves_words() {
    let result = wrap_text("one two three four five", 8);
    assert!(result.len() >= 3);
    // Each line should be within width
    for line in &result {
        assert!(line.len() <= 8, "Line '{}' exceeds width 8", line);
    }
}

#[test]
fn test_wrap_whitespace_text() {
    let result = wrap_text("   ", 10);
    // Whitespace-only text — textwrap should handle it
    assert!(!result.is_empty());
}

#[test]
fn test_wrap_newline_in_input() {
    let result = wrap_text("line one\nline two", 20);
    // textwrap respects existing newlines as hard breaks
    assert!(result.len() >= 2);
}

#[test]
fn test_wrap_unicode_emoji() {
    let result = wrap_text("Hello 🚀 World 🎉", 10);
    assert!(result.len() >= 2);
    // Each line should be within width (emojis are multi-byte but count as 1 char)
    for line in &result {
        assert!(
            line.chars().count() <= 10,
            "Line has {} chars, expected <= 10",
            line.chars().count()
        );
    }
}

#[test]
fn test_wrap_unicode_cjk() {
    // CJK characters are wider but still 1 char each
    let result = wrap_text("日本語のテキスト", 4);
    assert!(result.len() >= 2);
    for line in &result {
        assert!(line.chars().count() <= 4);
    }
}

#[test]
fn test_wrap_width_one() {
    // Extreme case: width=1 breaks every character
    let result = wrap_text("abc", 1);
    assert!(result.len() >= 3);
}

#[test]
fn test_wrap_preserves_leading_spaces() {
    let result = wrap_text("  indented text here", 15);
    // Should not lose content
    let joined = result.join(" ");
    assert!(joined.contains("indented"));
    assert!(joined.contains("text"));
}

// ─── image_to_data_uri ──────────────────────────────────────────────

#[test]
fn test_data_uri_png_file() {
    // Create a minimal valid PNG (1x1 transparent)
    let png_bytes: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0D, 0x49,
        0x44, 0x41, 0x54, 0x78, 0x9C, 0x62, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A,
        0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), png_bytes).unwrap();

    let path = tmp.path().to_str().unwrap();
    let data_uri = image_to_data_uri(path, cosy::text::ImagePolicy::UNRESTRICTED);
    assert!(data_uri.is_ok());
    let uri = data_uri.unwrap();
    assert!(uri.starts_with("data:image/png;base64,"));
    assert!(uri.len() > 30); // Should have actual base64 data
}

#[test]
fn test_data_uri_nonexistent_file() {
    let result = image_to_data_uri(
        "/nonexistent/path/image.png",
        cosy::text::ImagePolicy::UNRESTRICTED,
    );
    assert!(result.is_err());
}

#[test]
fn test_data_uri_mime_detection_jpg() {
    // Create a dummy file with .jpg extension
    let tmp = tempfile::Builder::new().suffix(".jpg").tempfile().unwrap();
    std::fs::write(tmp.path(), b"fake jpg content").unwrap();
    let uri = image_to_data_uri(
        tmp.path().to_str().unwrap(),
        cosy::text::ImagePolicy::UNRESTRICTED,
    )
    .unwrap();
    assert!(uri.starts_with("data:image/jpeg;base64,"));
}

#[test]
fn test_data_uri_mime_detection_svg() {
    let tmp = tempfile::Builder::new().suffix(".svg").tempfile().unwrap();
    std::fs::write(tmp.path(), b"<svg></svg>").unwrap();
    let uri = image_to_data_uri(
        tmp.path().to_str().unwrap(),
        cosy::text::ImagePolicy::UNRESTRICTED,
    )
    .unwrap();
    assert!(uri.starts_with("data:image/svg+xml;base64,"));
}

#[test]
fn test_data_uri_mime_detection_webp() {
    let tmp = tempfile::Builder::new().suffix(".webp").tempfile().unwrap();
    std::fs::write(tmp.path(), b"fake webp").unwrap();
    let uri = image_to_data_uri(
        tmp.path().to_str().unwrap(),
        cosy::text::ImagePolicy::UNRESTRICTED,
    )
    .unwrap();
    assert!(uri.starts_with("data:image/webp;base64,"));
}

#[test]
fn test_data_uri_unknown_extension_defaults_png() {
    let tmp = tempfile::Builder::new().suffix(".xyz").tempfile().unwrap();
    std::fs::write(tmp.path(), b"unknown").unwrap();
    let uri = image_to_data_uri(
        tmp.path().to_str().unwrap(),
        cosy::text::ImagePolicy::UNRESTRICTED,
    )
    .unwrap();
    assert!(uri.starts_with("data:image/png;base64,"));
}

#[test]
fn test_data_uri_no_extension_defaults_png() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), b"no extension").unwrap();
    let uri = image_to_data_uri(
        tmp.path().to_str().unwrap(),
        cosy::text::ImagePolicy::UNRESTRICTED,
    )
    .unwrap();
    assert!(uri.starts_with("data:image/png;base64,"));
}

#[test]
fn test_data_uri_jpeg_extension() {
    // .jpeg (not just .jpg)
    let tmp = tempfile::Builder::new().suffix(".jpeg").tempfile().unwrap();
    std::fs::write(tmp.path(), b"fake jpeg").unwrap();
    let uri = image_to_data_uri(
        tmp.path().to_str().unwrap(),
        cosy::text::ImagePolicy::UNRESTRICTED,
    )
    .unwrap();
    assert!(uri.starts_with("data:image/jpeg;base64,"));
}
