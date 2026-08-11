//! CLI argument parsing and command dispatch.
//!
//! Commands:
//! - `cosy render`     — render template + data → PNG
//! - `cosy templates`  — list available templates
//! - `cosy validate`   — validate input data against template schema

use clap::{Parser, Subcommand};
use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;

/// Cosy — Content Easy: Lightning-fast template-based image generation.
#[derive(Parser, Debug)]
#[command(name = "cosy", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Render images from a template + input data.
    Render {
        /// Template name (e.g. "social-quote") or path to template directory.
        #[arg(short, long)]
        template: String,

        /// Input data: path to JSON file.
        #[arg(short, long, conflicts_with_all = ["stdin", "json"])]
        data: Option<String>,

        /// Read input JSON from stdin.
        #[arg(long, conflicts_with_all = ["data", "json"])]
        stdin: bool,

        /// Inline JSON input string.
        #[arg(long, conflicts_with_all = ["data", "stdin"])]
        json: Option<String>,

        /// Output file (single slide) or directory (multi-slide).
        #[arg(short, long)]
        output: PathBuf,

        /// Scale factor (1 = normal, 2 = retina/2x).
        #[arg(long, default_value = "2")]
        scale: f32,

        /// Additional font directory to load.
        #[arg(long)]
        font_dir: Option<PathBuf>,

        /// Dump the processed SVG to stdout instead of rendering PNG.
        #[arg(long)]
        dump_svg: bool,

        /// Output machine-readable JSON result to stdout (logging goes to stderr).
        #[arg(long)]
        json_output: bool,
    },

    /// List available templates.
    Templates {
        /// Template directory to scan.
        #[arg(long, default_value = "./templates")]
        dir: PathBuf,

        /// Output as JSON.
        #[arg(long)]
        json: bool,
    },

    /// Validate input data against a template schema.
    Validate {
        /// Template name or path.
        #[arg(short, long)]
        template: String,

        /// Input data JSON file.
        #[arg(short, long)]
        data: String,
    },
}

impl Cli {
    pub fn run(self) -> anyhow::Result<ExitCode> {
        match self.command {
            Command::Render {
                template,
                data,
                stdin,
                json,
                output,
                scale,
                font_dir,
                dump_svg,
                json_output,
            } => {
                // Resolve input data source
                let resolved_data = match Self::resolve_input(data, stdin, json)? {
                    Some(d) => d,
                    None => {
                        eprintln!("✗ Error: must provide one of --data, --stdin, or --json");
                        return Ok(ExitCode::from(2));
                    }
                };

                if dump_svg {
                    return dump_processed_svg(&template, &resolved_data);
                }

                match crate::render::render_template(
                    &template,
                    &resolved_data,
                    &output,
                    scale,
                    font_dir.as_deref(),
                ) {
                    Ok(result) => {
                        if json_output {
                            // Machine-readable output to stdout
                            let json_out = serde_json::to_string_pretty(&result)?;
                            println!("{}", json_out);
                        }
                        Ok(ExitCode::SUCCESS)
                    }
                    Err(e) => {
                        if json_output {
                            let err = serde_json::json!({
                                "error": format!("{:#}", e),
                                "code": 1
                            });
                            println!("{}", serde_json::to_string_pretty(&err)?);
                        } else {
                            eprintln!("✗ Render error: {:#}", e);
                        }
                        Ok(ExitCode::from(2))
                    }
                }
            }

            Command::Templates { dir, json } => {
                let templates = crate::template::list_templates(&dir);

                if json {
                    let json_out = serde_json::to_string_pretty(&templates)?;
                    println!("{}", json_out);
                } else {
                    if templates.is_empty() {
                        println!("No templates found in {}", dir.display());
                    } else {
                        println!("Available templates ({}):", templates.len());
                        println!();
                        for t in &templates {
                            println!(
                                "  {:20} {:40} {}×{}",
                                t.id, t.name, t.dimensions.width, t.dimensions.height
                            );
                        }
                    }
                }
                Ok(ExitCode::SUCCESS)
            }

            Command::Validate { template, data } => {
                let tmpl = match crate::template::load_template(&template) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("✗ Failed to load template: {:#}", e);
                        return Ok(ExitCode::from(2));
                    }
                };

                let input = match crate::schema::InputData::from_file(&data) {
                    Ok(d) => d,
                    Err(e) => {
                        eprintln!("✗ Failed to load data: {:#}", e);
                        return Ok(ExitCode::from(2));
                    }
                };

                let errors = crate::template::validate_input(&tmpl, &input);
                if errors.is_empty() {
                    println!(
                        "✓ Valid! {} slide(s), all fields OK for template '{}'",
                        input.slides.len(),
                        tmpl.name
                    );
                    Ok(ExitCode::SUCCESS)
                } else {
                    eprintln!("✗ Validation failed ({} error(s)):", errors.len());
                    for e in &errors {
                        eprintln!("  - {}", e);
                    }
                    Ok(ExitCode::from(1))
                }
            }
        }
    }

    /// Resolve input data from --data (file), --stdin, or --json (inline string).
    /// Returns Some(json_string) or None if no source provided.
    fn resolve_input(
        data: Option<String>,
        stdin: bool,
        json: Option<String>,
    ) -> anyhow::Result<Option<String>> {
        if let Some(path) = data {
            // --data: treat as file path
            Ok(Some(path))
        } else if stdin {
            // --stdin: read from stdin, write to temp file for from_file compatibility
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            let tmp = std::env::temp_dir().join("cosy-stdin-input.json");
            std::fs::write(&tmp, &buffer)?;
            Ok(Some(tmp.to_string_lossy().to_string()))
        } else if let Some(json_str) = json {
            // --json: write inline JSON to temp file
            let tmp = std::env::temp_dir().join("cosy-json-input.json");
            std::fs::write(&tmp, &json_str)?;
            Ok(Some(tmp.to_string_lossy().to_string()))
        } else {
            Ok(None)
        }
    }
}

/// Debug helper: dump the processed SVG after minijinja rendering.
fn dump_processed_svg(template_name: &str, data_path: &str) -> anyhow::Result<ExitCode> {
    let template = crate::template::load_template(template_name)?;
    let dir = crate::template::find_template_dir_for(template_name)?;
    let data = crate::schema::InputData::from_file(data_path)?;
    let svg = crate::template::process_template(&template, &dir, &data.brand, &data.slides[0])?;
    println!("{}", svg);
    Ok(ExitCode::SUCCESS)
}
