//! Text processing utilities: wrapping, image encoding.

use base64::Engine;
use std::path::Path;
use std::time::Duration;

/// Refuse to download remote images larger than this (10 MB).
const MAX_REMOTE_IMAGE_BYTES: u64 = 10 * 1024 * 1024;

/// Timeout for fetching a remote image.
const FETCH_TIMEOUT: Duration = Duration::from_secs(10);

/// Wrap text to fit within a max character width per line.
/// Uses textwrap crate for word-boundary-aware wrapping.
/// Words longer than max_chars are broken at character limit.
/// Empty text returns a single empty string element.
pub fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }

    let options = textwrap::Options::new(max_chars)
        .break_words(true)
        .word_separator(textwrap::WordSeparator::AsciiSpace);

    textwrap::wrap(text, &options)
        .into_iter()
        .map(|cow| cow.into_owned())
        .collect()
}

/// Convert an image to a base64 data URI for SVG embedding.
///
/// Accepts a local file path or an `http(s)://` URL. Remote images are
/// fetched synchronously with a 10 s timeout and a 10 MB size cap; the
/// server render path must call this from a blocking thread
/// (`tokio::task::spawn_blocking`).
pub fn image_to_data_uri(path: &str) -> anyhow::Result<String> {
    if is_remote_url(path) {
        return fetch_image_data_uri(path);
    }

    let mime = mime_for_extension(
        Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("png"),
    );

    let bytes = std::fs::read(path)?;
    encode_data_uri(&bytes, mime)
}

/// True if the value looks like an http(s) URL rather than a file path.
pub fn is_remote_url(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}

fn encode_data_uri(bytes: &[u8], mime: &str) -> anyhow::Result<String> {
    let b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
    Ok(format!("data:{};base64,{}", mime, b64))
}

fn mime_for_extension(ext: &str) -> &'static str {
    match ext.to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        _ => "image/png",
    }
}

/// Download a remote image and encode it as a data URI.
fn fetch_image_data_uri(url: &str) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::builder()
        .timeout(FETCH_TIMEOUT)
        .build()?;

    let response = client.get(url).send()?.error_for_status()?;

    if let Some(len) = response.content_length() {
        if len > MAX_REMOTE_IMAGE_BYTES {
            anyhow::bail!(
                "remote image at {} is {} bytes, limit is {}",
                url,
                len,
                MAX_REMOTE_IMAGE_BYTES
            );
        }
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|ct| ct.split(';').next().unwrap_or_default().trim().to_string());

    let bytes = response.bytes()?;
    if bytes.len() as u64 > MAX_REMOTE_IMAGE_BYTES {
        anyhow::bail!(
            "remote image at {} is {} bytes, limit is {}",
            url,
            bytes.len(),
            MAX_REMOTE_IMAGE_BYTES
        );
    }

    let mime = content_type
        .filter(|ct| ct.starts_with("image/"))
        .map(|ct| ct.to_string())
        .unwrap_or_else(|| {
            let ext = Path::new(
                url.split(['?', '#']).next().unwrap_or(url),
            )
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("png");
            mime_for_extension(ext).to_string()
        });

    encode_data_uri(&bytes, &mime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_wrap() {
        let result = wrap_text("Build things that solve real problems", 20);
        assert!(result.len() >= 2);
        assert!(result.iter().all(|l| l.len() <= 20));
    }

    #[test]
    fn test_empty_text() {
        let result = wrap_text("", 40);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_long_word_break() {
        let result = wrap_text("supercalifragilisticexpialidocious", 10);
        // Should break the long word
        assert!(result.len() >= 3);
        assert!(result.iter().all(|l| l.len() <= 10));
    }

    #[test]
    fn test_short_text_no_wrap() {
        let result = wrap_text("Hello world", 100);
        assert_eq!(result, vec!["Hello world"]);
    }

    #[test]
    fn test_indonesian_text() {
        let result = wrap_text("Membangun produk yang berguna untuk semua orang", 30);
        assert!(result.len() >= 2);
        assert!(result.iter().all(|l| l.len() <= 30));
    }

    /// Serve one canned HTTP response from 127.0.0.1 and return its URL.
    fn serve_one(response: String) -> String {
        use std::io::{Read, Write};

        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut buf = [0u8; 2048];
            let _ = stream.read(&mut buf);
            stream.write_all(response.as_bytes()).unwrap();
        });
        format!("http://{addr}/img.png")
    }

    fn response_with(content_type: &str, body: &[u8]) -> String {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            content_type,
            body.len()
        ) + std::str::from_utf8(body).unwrap()
    }

    #[test]
    fn test_is_remote_url() {
        assert!(is_remote_url("https://example.com/a.png"));
        assert!(is_remote_url("http://localhost:8080/b.webp"));
        assert!(!is_remote_url("/path/to/c.png"));
        assert!(!is_remote_url("C:\\img\\d.png"));
        assert!(!is_remote_url("ftp://example.com/e.png"));
    }

    #[test]
    fn test_fetch_url_success_uses_content_type() {
        let url = serve_one(response_with("image/png", b"fakepngbytes"));
        let uri = image_to_data_uri(&url).expect("fetch must succeed");
        assert!(uri.starts_with("data:image/png;base64,"), "got: {uri}");
    }

    #[test]
    fn test_fetch_url_falls_back_to_url_extension() {
        // application/octet-stream is not an image/* → mime from .png extension
        let url = serve_one(response_with("application/octet-stream", b"jpegdata"));
        let uri = image_to_data_uri(&url).unwrap();
        assert!(uri.starts_with("data:image/png;base64,"), "got: {uri}");
    }

    #[test]
    fn test_fetch_url_rejects_oversized_content_length() {
        let url = serve_one(
            "HTTP/1.1 200 OK\r\nContent-Type: image/png\r\nContent-Length: 99999999999\r\nConnection: close\r\n\r\n"
                .to_string(),
        );
        let err = image_to_data_uri(&url).expect_err("must reject oversized image");
        assert!(err.to_string().contains("limit"), "got: {err}");
    }

    #[test]
    fn test_fetch_url_rejects_http_error_status() {
        let url = serve_one(
            "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string(),
        );
        assert!(image_to_data_uri(&url).is_err(), "404 must be an error");
    }
}
