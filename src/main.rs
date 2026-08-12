//! Cosy — Content Easy binary entry point.

use clap::Parser;
use cosy::cli::Cli;

fn main() -> std::process::ExitCode {
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
