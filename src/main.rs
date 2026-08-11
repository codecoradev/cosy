//! Cosy — Content Easy
//! Template-based image generation in Rust.
//!
//! Pipeline: JSON input → minijinja token replacement → SVG → resvg render → PNG

mod cli;
mod schema;
mod render;
mod template;
mod text;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()
}
