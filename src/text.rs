//! Text processing utilities: wrapping, image encoding.

use std::path::Path;
use base64::Engine;

/// Wrap text to fit within a max character width per line
pub fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    textwrap::wrap(text, max_chars)
        .into_iter()
        .map(|cow| cow.into_owned())
        .collect()
}

/// Convert an image file to a base64 data URI for SVG embedding
pub fn image_to_data_uri(path: &str) -> anyhow::Result<String> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("png");

    let mime = match ext.to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        _ => "image/png",
    };

    let bytes = std::fs::read(path)?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    Ok(format!("data:{};base64,{}", mime, b64))
}
