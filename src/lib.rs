//! Cosy — Content Easy
//! Template-based image generation in Rust.
//!
//! Pipeline: JSON input → minijinja token replacement → SVG → resvg render → PNG

pub mod cli;
pub mod render;
pub mod schema;
pub mod server;
pub mod template;
pub mod text;
