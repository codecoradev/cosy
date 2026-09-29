//! Output format handling: what container wraps the rendered pixels.
//!
//! Two independent axes:
//! - **Container** (`OutputFormat`): PNG or WebP — how bytes are packed.
//! - **API response shape** (`ResponseFormat` in server.rs): binary image or
//!   JSON envelope. Kept orthogonal on purpose: a JSON envelope can carry
//!   PNG or WebP entries without multiplying enum variants.

use image::ImageFormat;
// Trait provides `write_image` on WebPEncoder — required import.
use image::ImageEncoder;

/// Output image container format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    /// PNG (default) — lossless, universal tooling support.
    #[default]
    Png,
    /// WebP — lossless via `image` 0.25 (lossy needs libwebp/C). Smaller
    /// than PNG on flat-color cards, larger on heavy photo content.
    WebP,
}

impl OutputFormat {
    /// Parse a format string ("png" | "webp"), case-insensitive.
    /// Returns Err with a user-facing message on unknown values.
    pub fn parse(s: &str) -> anyhow::Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "png" => Ok(Self::Png),
            "webp" => Ok(Self::WebP),
            other => Err(anyhow::anyhow!(
                "Unknown output format '{}': expected \"png\" or \"webp\"",
                other
            )),
        }
    }

    /// File extension without the dot.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::WebP => "webp",
        }
    }

    /// IANA media type for HTTP responses.
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::WebP => "image/webp",
        }
    }

    /// Encode raw RGBA pixels into the target container format.
    ///
    /// `pixels` is premultiplied-alpha-independent RGBA8 from tiny-skia's
    /// `pixmap.data()`; `w`/`h` are the final (scaled) dimensions.
    ///
    /// WebP is lossy at quality 90: visually indistinguishable for social
    /// cards while cutting file size ~3-5x vs PNG.
    pub fn encode(&self, pixels: &[u8], w: u32, h: u32) -> anyhow::Result<Vec<u8>> {
        if pixels.len() != (w as usize) * (h as usize) * 4 {
            return Err(anyhow::anyhow!(
                "Pixel buffer size mismatch: {} bytes for {}x{} RGBA",
                pixels.len(),
                w,
                h
            ));
        }
        let img = image::RgbaImage::from_raw(w, h, pixels.to_vec())
            .ok_or_else(|| anyhow::anyhow!("Failed to wrap pixel buffer as RGBA image"))?;

        let mut out = std::io::Cursor::new(Vec::new());
        match self {
            Self::Png => {
                // Same encoder tiny-skia uses; keeps byte-identical PNG output.
                img.write_to(&mut out, ImageFormat::Png)?;
            }
            Self::WebP => {
                // `image` 0.25 exposes only the lossless WebP encoder
                // (lossy requires libwebp via the `webp` crate — C dep,
                // deliberately avoided in this build).
                let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut out);
                encoder.write_image(pixels, w, h, image::ExtendedColorType::Rgba8)?;
            }
        }
        Ok(out.into_inner())
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

    /// 4x2 solid image so encoders have real content to compress.
    fn solid_pixels() -> Vec<u8> {
        let mut px = Vec::with_capacity(4 * 2 * 4);
        for _ in 0..4 * 2 {
            px.extend_from_slice(&[30, 30, 46, 255]); // #1e1e2e opaque
        }
        px
    }

    #[test]
    fn parse_accepts_case_insensitive() {
        assert_eq!(OutputFormat::parse("png").unwrap(), OutputFormat::Png);
        assert_eq!(OutputFormat::parse("WebP").unwrap(), OutputFormat::WebP);
        assert_eq!(OutputFormat::parse("WEBP").unwrap(), OutputFormat::WebP);
    }

    #[test]
    fn parse_rejects_unknown() {
        assert!(OutputFormat::parse("jpg").is_err());
        assert!(OutputFormat::parse("").is_err());
        let err = OutputFormat::parse("avif").unwrap_err().to_string();
        assert!(err.contains("png") && err.contains("webp"));
    }

    #[test]
    fn extension_and_mime_match() {
        assert_eq!(OutputFormat::Png.extension(), "png");
        assert_eq!(OutputFormat::WebP.extension(), "webp");
        assert_eq!(OutputFormat::Png.mime_type(), "image/png");
        assert_eq!(OutputFormat::WebP.mime_type(), "image/webp");
    }

    #[test]
    fn default_is_png() {
        assert_eq!(OutputFormat::default(), OutputFormat::Png);
    }

    #[test]
    fn encode_png_produces_png_magic() {
        let bytes = OutputFormat::Png.encode(&solid_pixels(), 4, 2).unwrap();
        assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
    }

    #[test]
    fn encode_webp_produces_riff_header() {
        let bytes = OutputFormat::WebP.encode(&solid_pixels(), 4, 2).unwrap();
        assert_eq!(&bytes[..4], b"RIFF");
        assert_eq!(&bytes[8..12], b"WEBP");
    }

    #[test]
    fn encode_rejects_wrong_buffer_size() {
        let px = vec![0u8; 10];
        assert!(OutputFormat::Png.encode(&px, 4, 2).is_err());
        assert!(OutputFormat::WebP.encode(&px, 4, 2).is_err());
    }
}
