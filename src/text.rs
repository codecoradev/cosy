//! Text processing utilities: wrapping, image encoding.

use base64::Engine;
use std::path::Path;

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

/// Convert an image file to a base64 data URI for SVG embedding.
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
}
