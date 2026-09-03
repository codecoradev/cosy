//! Text processing utilities: wrapping, image encoding.

use base64::Engine;
use std::io::Read;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// Refuse to download remote images larger than this (10 MB).
const MAX_REMOTE_IMAGE_BYTES: u64 = 10 * 1024 * 1024;

/// Timeout for fetching a remote image.
const FETCH_TIMEOUT: Duration = Duration::from_secs(10);

/// Whether private/internal addresses may be fetched. Defaults to `false` so
/// the HTTP server cannot be abused for SSRF against internal services.
/// The standalone CLI enables it (a local user is already trusted), the
/// server only via the explicit `--allow-private-images` flag.
static ALLOW_PRIVATE_IMAGES: AtomicBool = AtomicBool::new(false);

pub fn set_allow_private_images(allow: bool) {
    ALLOW_PRIVATE_IMAGES.store(allow, Ordering::SeqCst);
}

fn allow_private_images() -> bool {
    ALLOW_PRIVATE_IMAGES.load(Ordering::SeqCst)
}

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
        return fetch_image_data_uri(path, allow_private_images());
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
///
/// Security posture (the caller decides via `allow_private`; the server
/// default is `false`):
/// - only `https://` URLs are accepted by default (`http://` requires
///   `allow_private`, i.e. the local CLI or the explicit server opt-in flag)
/// - the host is resolved up front and the connection is **pinned to the
///   validated address**, so DNS cannot rotate to a private target between
///   validation and connection (DNS rebinding)
/// - only globally routable addresses pass: loopback, RFC1918, link-local
///   (incl. cloud metadata `169.254.169.254`), CGNAT, and other special-use
///   ranges are rejected
/// - redirects are not followed; a redirect response is an error
/// - the body is streamed with a hard `take()` cap — chunked/lying
///   Content-Length cannot balloon memory
/// - failures log details server-side and return a generic message
fn fetch_image_data_uri(url_str: &str, allow_private: bool) -> anyhow::Result<String> {
    let url = reqwest::Url::parse(url_str)?;
    let scheme_ok = url.scheme() == "https" || (allow_private && url.scheme() == "http");
    if !scheme_ok {
        log::warn!("rejected image URL scheme for {url_str}");
        anyhow::bail!("failed to load remote image");
    }

    let mut builder = reqwest::blocking::Client::builder()
        .timeout(FETCH_TIMEOUT)
        .redirect(reqwest::redirect::Policy::none());
    if !allow_private {
        let host = url
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("failed to load remote image"))?;
        let addr = validated_public_addr(&url)?;
        builder = builder.resolve(host, addr);
    }
    let client = builder.build()?;

    let response = match client.get(url.clone()).send() {
        Ok(r) => r,
        Err(e) => {
            log::warn!("remote image fetch failed for {url_str}: {e}");
            anyhow::bail!("failed to load remote image");
        }
    };

    let response = match response.error_for_status() {
        Ok(r) => r,
        Err(e) => {
            log::warn!("remote image fetch returned an error status for {url_str}: {e}");
            anyhow::bail!("failed to load remote image");
        }
    };
    // redirects are disabled; a 3xx that slipped past the policy is an error
    if response.status().is_redirection() {
        log::warn!("remote image redirect not followed for {url_str}");
        anyhow::bail!("failed to load remote image");
    }

    if let Some(len) = response.content_length() {
        if len > MAX_REMOTE_IMAGE_BYTES {
            anyhow::bail!(
                "remote image exceeds the {} MB size limit",
                MAX_REMOTE_IMAGE_BYTES / 1024 / 1024
            );
        }
    }

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|ct| ct.split(';').next().unwrap_or_default().trim().to_string());

    let mut bytes: Vec<u8> = Vec::new();
    if let Err(e) = response
        .take(MAX_REMOTE_IMAGE_BYTES + 1)
        .read_to_end(&mut bytes)
    {
        log::warn!("remote image download interrupted for {url_str}: {e}");
        anyhow::bail!("failed to load remote image");
    }
    if bytes.len() as u64 > MAX_REMOTE_IMAGE_BYTES {
        anyhow::bail!(
            "remote image exceeds the {} MB size limit",
            MAX_REMOTE_IMAGE_BYTES / 1024 / 1024
        );
    }

    let mime = content_type
        .filter(|ct| ct.starts_with("image/"))
        .map(|ct| ct.to_string())
        .unwrap_or_else(|| {
            let ext = Path::new(url_str.split(['?', '#']).next().unwrap_or(url_str))
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("png");
            mime_for_extension(ext).to_string()
        });

    encode_data_uri(&bytes, &mime)
}

/// Resolve the URL host and return its first globally routable address.
/// Rejects URLs whose hosts resolve only to non-public targets.
fn validated_public_addr(url: &reqwest::Url) -> anyhow::Result<SocketAddr> {
    let host = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("failed to load remote image"))?;
    let port = url.port_or_known_default().unwrap_or(80);
    let addr = (host, port)
        .to_socket_addrs()?
        .find(|addr| is_public_ip(addr.ip()));
    match addr {
        Some(addr) => Ok(addr),
        None => {
            log::warn!("blocked image fetch to non-public address (host: {host})");
            anyhow::bail!("failed to load remote image")
        }
    }
}

/// True only for globally routable addresses. Covers loopback, RFC1918
/// private ranges, link-local (incl. cloud metadata endpoints), CGNAT,
/// documentation/benchmark ranges, multicast, and their IPv6 equivalents
/// (unique-local, link-local, multicast) and IPv4-mapped IPv6 addresses.
fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            let o = v4.octets();
            !(v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local()
                || v4.is_broadcast()
                || v4.is_documentation()
                || v4.is_unspecified()
                || o[0] == 0
                || (o[0] == 100 && (64..=127).contains(&o[1]))
                || (o[0] == 198 && (18..=19).contains(&o[1]))
                || o[0] >= 224)
        }
        IpAddr::V6(v6) => {
            // to_ipv4() unwraps both IPv4-mapped (::ffff:a.b.c.d) and the
            // deprecated IPv4-compatible (::a.b.c.d) forms, e.g. ::127.0.0.1
            if let Some(v4) = v6.to_ipv4() {
                return is_public_ip(IpAddr::V4(v4));
            }
            !(v6.is_loopback()
                || v6.is_unspecified()
                || (v6.segments()[0] & 0xfe00) == 0xfc00
                || (v6.segments()[0] & 0xffc0) == 0xfe80
                || (v6.segments()[0] & 0xff00) == 0xff00)
        }
    }
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
        let uri = fetch_image_data_uri(&url, true).expect("fetch must succeed");
        assert!(uri.starts_with("data:image/png;base64,"), "got: {uri}");
    }

    #[test]
    fn test_fetch_url_falls_back_to_url_extension() {
        // application/octet-stream is not an image/* → mime from .png extension
        let url = serve_one(response_with("application/octet-stream", b"jpegdata"));
        let uri = fetch_image_data_uri(&url, true).unwrap();
        assert!(uri.starts_with("data:image/png;base64,"), "got: {uri}");
    }

    #[test]
    fn test_fetch_url_rejects_oversized_content_length() {
        let url = serve_one(
            "HTTP/1.1 200 OK\r\nContent-Type: image/png\r\nContent-Length: 99999999999\r\nConnection: close\r\n\r\n"
                .to_string(),
        );
        let err = fetch_image_data_uri(&url, true).expect_err("must reject oversized image");
        assert!(err.to_string().contains("limit"), "got: {err}");
    }

    #[test]
    fn test_fetch_url_rejects_http_error_status() {
        let url = serve_one(
            "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_string(),
        );
        assert!(
            fetch_image_data_uri(&url, true).is_err(),
            "404 must be an error"
        );
    }

    #[test]
    fn test_fetch_url_blocks_private_targets_by_default() {
        let url = serve_one(response_with("image/png", b"secret"));
        let err = fetch_image_data_uri(&url, false)
            .expect_err("loopback must be blocked without allow_private");
        assert_eq!(err.to_string(), "failed to load remote image");
    }

    #[test]
    fn test_fetch_url_does_not_follow_redirects() {
        // redirects are disabled entirely: a 3xx is an error in both modes
        let url = serve_one(
            "HTTP/1.1 302 Found\r\nLocation: http://127.0.0.1:9/x\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
                .to_string(),
        );
        assert!(fetch_image_data_uri(&url, false).is_err());
        assert!(fetch_image_data_uri(&url, true).is_err());
    }

    #[test]
    fn test_fetch_url_https_only_by_default() {
        // plain http requires allow_private (local CLI / server opt-in flag)
        let url = serve_one(response_with("image/png", b"data"));
        assert!(fetch_image_data_uri(&url, false).is_err());
        assert!(fetch_image_data_uri(&url, true).is_ok());
    }

    #[test]
    fn test_fetch_url_rejects_non_http_schemes() {
        assert!(fetch_image_data_uri("file:///etc/passwd", true).is_err());
        assert!(fetch_image_data_uri("ftp://example.com/a.png", true).is_err());
    }

    #[test]
    fn test_fetch_url_localhost_hostname_blocked_by_default() {
        // hostname (not IP literal) resolving to loopback must also be blocked
        let url = serve_one(response_with("image/png", b"data"));
        let host_url = url.replace("127.0.0.1", "localhost");
        assert!(fetch_image_data_uri(&host_url, false).is_err());
    }

    #[test]
    fn test_is_public_ip_classification() {
        // private / special-use → not public
        for ip in [
            "127.0.0.1",
            "10.1.2.3",
            "172.16.0.9",
            "192.168.1.1",
            "169.254.169.254",
            "0.0.0.0",
            "100.64.0.1",
            "198.18.0.5",
            "224.0.0.1",
            "250.1.2.3",
            "::1",
            "::",
            "fc00::1",
            "fe80::1",
            "ff02::1",
            "::ffff:10.0.0.1",
            "::ffff:127.0.0.1",
            "::127.0.0.1",
            "::169.254.169.254",
            "::10.0.0.1",
        ] {
            let ip: IpAddr = ip.parse().unwrap();
            assert!(!is_public_ip(ip), "{ip} must not be public");
        }
        // globally routable → public
        for ip in ["8.8.8.8", "1.1.1.1", "172.32.0.1", "2606:4700::1111"] {
            let ip: IpAddr = ip.parse().unwrap();
            assert!(is_public_ip(ip), "{ip} must be public");
        }
    }
}
