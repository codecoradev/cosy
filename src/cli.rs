//! CLI argument parsing and command dispatch.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Cosy — Content Easy: Lightning-fast template-based image generation
#[derive(Parser, Debug)]
#[command(name = "cosy", version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Render images from a template
    Render {
        /// Template name or path
        #[arg(short, long)]
        template: String,

        /// Input data (JSON file path or inline JSON string)
        #[arg(short, long)]
        data: String,

        /// Output file or directory (for multi-slide)
        #[arg(short, long)]
        output: PathBuf,

        /// Scale factor (1 = normal, 2 = retina)
        #[arg(long, default_value = "2")]
        scale: f32,
    },

    /// List available templates
    Templates {
        /// Template directory
        #[arg(long, default_value = "./templates")]
        dir: PathBuf,
    },

    /// Validate a template schema
    Validate {
        /// Template name or path
        #[arg(short, long)]
        template: String,

        /// Input data to validate against
        #[arg(short, long)]
        data: String,
    },
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            Command::Render { template, data, output, scale } => {
                crate::render::render_template(&template, &data, &output, scale)
            }
            Command::Templates { dir } => {
                crate::template::list_templates(&dir)
            }
            Command::Validate { template, data } => {
                crate::template::validate_template(&template, &data)
            }
        }
    }
}
