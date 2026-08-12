//! Cosy — Content Easy
//! Template-based image generation in Rust.
//!
//! Pipeline: JSON input → minijinja token replacement → SVG → resvg render → PNG

mod cli;
mod render;
mod schema;
mod server;
mod template;
mod text;

use clap::Parser;
use cli::Cli;

fn main() -> std::process::ExitCode {
    // Initialize logger (RUST_LOG=cosy=debug for verbose output)
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
        .format_timestamp(None)
        .init();

    let cli = Cli::parse();

    match cli.run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("✗ Fatal: {:#}", e);
            std::process::ExitCode::from(2)
        }
    }
}
