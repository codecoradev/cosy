//! CLI argument parsing and command dispatch.
//!
//! Commands:
//! - `cosy render`     — render template + data → PNG
//! - `cosy templates`  — list available templates
//! - `cosy validate`   — validate input data against template schema

use clap::{Parser, Subcommand, ValueEnum};
use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;

/// Output container format for `cosy render`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormatArg {
    /// PNG (default) — lossless.
    Png,
    /// WebP — lossless container via the `image` crate.
    Webp,
    /// SVG — vector-native, text converted to paths (scale ignored).
    Svg,
}

impl From<OutputFormatArg> for crate::format::OutputFormat {
    fn from(arg: OutputFormatArg) -> Self {
        match arg {
            OutputFormatArg::Png => Self::Png,
            OutputFormatArg::Webp => Self::WebP,
            OutputFormatArg::Svg => Self::Svg,
        }
    }
}

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

        /// Output image container format (PNG default, WebP lossless).
        #[arg(long, value_enum, default_value = "png")]
        format: OutputFormatArg,

        /// Optional metadata JSON string echoed in the --json-output result
        /// (pipeline tracing / correlation IDs).
        #[arg(long)]
        metadata: Option<String>,

        /// CSV dataset file: one render per row (header = field names).
        /// Requires --output to be a directory.
        #[arg(long, conflicts_with_all = ["data", "stdin", "json"])]
        dataset: Option<PathBuf>,

        /// With --dataset: stop scheduling new rows after the first failure.
        #[arg(long, requires = "dataset")]
        fail_fast: bool,
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

    /// Start HTTP API server.
    Serve {
        /// Port to listen on.
        #[arg(short, long, default_value = "3000")]
        port: u16,

        /// Address to bind. Defaults to all interfaces (needed in Docker);
        /// use 127.0.0.1 for local dev without auth.
        #[arg(long, default_value = "0.0.0.0")]
        host: String,

        /// API key for bearer token auth. If not set, reads COSY_API_KEY env var.
        /// When neither is set, auth is disabled (dev mode).
        #[arg(short, long)]
        token: Option<String>,

        /// HMAC key for signed GET render URLs (`/r/...`). If not set, reads
        /// COSY_SIGNING_KEY, then falls back to the API key.
        #[arg(long)]
        signing_key: Option<String>,

        /// Allow bg_image/logo URLs pointing at private/internal addresses.
        /// Off by default: the API renders attacker-controlled JSON, so
        /// image fetches to loopback/RFC1918/link-local targets are blocked.
        #[arg(long)]
        allow_private_images: bool,

        /// Allow bg_image/logo values referencing local filesystem paths.
        /// Off by default: the API renders attacker-controlled JSON, so
        /// local paths would expose server files through the render output.
        #[arg(long)]
        allow_local_image_paths: bool,
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
                format,
                metadata,
                dataset,
                fail_fast,
            } => {
                // Local CLI runs are user-driven: no image-source restrictions.
                let image_policy = crate::text::ImagePolicy::UNRESTRICTED;

                // CSV batch mode: one render per row, results summarized.
                // Independent of --data/--stdin/--json (each row carries its
                // own data); template + output are validated here.
                if let Some(csv_path) = dataset {
                    // Load template up front for a clear error message.
                    if let Err(e) = crate::template::load_template(&template) {
                        eprintln!("✗ Failed to load template: {:#}", e);
                        return Ok(ExitCode::from(2));
                    }
                    let batch = crate::batch::render_csv(
                        &template,
                        &csv_path,
                        &output,
                        scale,
                        font_dir.as_deref(),
                        image_policy,
                        format.into(),
                        fail_fast,
                    )?;
                    if json_output {
                        println!("{}", serde_json::to_string_pretty(&batch)?);
                    } else {
                        println!(
                            "Batch complete: {}/{} rows OK ({} failed)",
                            batch.succeeded, batch.total_rows, batch.failed
                        );
                        for e in &batch.errors {
                            eprintln!("  - {}", e);
                        }
                    }
                    return Ok(if batch.failed > 0 {
                        ExitCode::from(1)
                    } else {
                        ExitCode::SUCCESS
                    });
                }

                // Resolve input data source
                let resolved_data = match Self::resolve_input(data, stdin, json)? {
                    Some(d) => d,
                    None => {
                        eprintln!("✗ Error: must provide one of --data, --stdin, or --json");
                        return Ok(ExitCode::from(2));
                    }
                };

                // Validate input against the template schema before rendering.
                // Fails fast with a clear field-level message instead of a
                // confusing Tera "invalid float literal" engine error.
                let tmpl_def = match crate::template::load_template(&template) {
                    Ok(t) => t,
                    Err(e) => {
                        if json_output {
                            let err = serde_json::json!({
                                "error": format!("{:#}", e),
                                "code": 1
                            });
                            println!("{}", serde_json::to_string_pretty(&err)?);
                        } else {
                            eprintln!("✗ Failed to load template: {:#}", e);
                        }
                        return Ok(ExitCode::from(2));
                    }
                };
                let input_data = match crate::schema::InputData::from_file(&resolved_data) {
                    Ok(d) => d,
                    Err(e) => {
                        if json_output {
                            let err = serde_json::json!({
                                "error": format!("{:#}", e),
                                "code": 1
                            });
                            println!("{}", serde_json::to_string_pretty(&err)?);
                        } else {
                            eprintln!("✗ Failed to parse data: {:#}", e);
                        }
                        return Ok(ExitCode::from(2));
                    }
                };
                let errors = crate::template::validate_input(&tmpl_def, &input_data);
                if !errors.is_empty() {
                    if json_output {
                        let err = serde_json::json!({
                            "error": format!("validation failed ({} error(s)): {}",
                                errors.len(), errors.join("; ")),
                            "code": 1
                        });
                        println!("{}", serde_json::to_string_pretty(&err)?);
                    } else {
                        eprintln!("✗ Validation failed ({} error(s)):", errors.len());
                        for e in &errors {
                            eprintln!("  - {}", e);
                        }
                    }
                    return Ok(ExitCode::from(1));
                }

                if dump_svg {
                    return dump_processed_svg(&template, &resolved_data);
                }

                // Parse optional metadata JSON up front so malformed input
                // fails fast, before any rendering work.
                let parsed_metadata = match &metadata {
                    Some(s) => match serde_json::from_str::<serde_json::Value>(s) {
                        Ok(v) => Some(v),
                        Err(e) => {
                            eprintln!("✗ Invalid --metadata JSON: {e}");
                            return Ok(ExitCode::from(2));
                        }
                    },
                    None => None,
                };

                match crate::render::render_template(
                    &template,
                    &resolved_data,
                    &output,
                    scale,
                    font_dir.as_deref(),
                    image_policy,
                    format.into(),
                ) {
                    Ok(result) => {
                        if json_output {
                            // Machine-readable output to stdout, with the
                            // caller's metadata echoed when provided.
                            let mut json_out = serde_json::to_value(&result)?;
                            if let Some(meta) = parsed_metadata {
                                json_out["metadata"] = meta;
                            }
                            println!("{}", serde_json::to_string_pretty(&json_out)?);
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
                } else if templates.is_empty() {
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

            Command::Serve {
                port,
                host,
                token,
                signing_key,
                allow_private_images,
                allow_local_image_paths,
            } => {
                // Resolve API key: --token flag takes priority, then COSY_API_KEY env
                let api_key = token
                    .or_else(|| std::env::var("COSY_API_KEY").ok())
                    .filter(|k| !k.is_empty());
                let signing_key = signing_key.or_else(|| std::env::var("COSY_SIGNING_KEY").ok());
                let image_policy = crate::text::ImagePolicy {
                    allow_private: allow_private_images,
                    allow_local: allow_local_image_paths,
                };

                if api_key.is_some() {
                    println!("🔒 Auth enabled — bearer token required");
                } else {
                    eprintln!("⚠️  Auth disabled — set --token or COSY_API_KEY to secure the API");
                }
                println!("Starting Cosy API server on port {}...", port);
                // Tokio runtime for async server
                let runtime = tokio::runtime::Runtime::new()?;
                runtime.block_on(crate::server::run_with(
                    &host,
                    port,
                    api_key,
                    signing_key,
                    image_policy,
                ))?;
                Ok(ExitCode::SUCCESS)
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
    let svg = crate::template::process_template(
        &template,
        &dir,
        &data.brand,
        &data.slides[0],
        crate::text::ImagePolicy::UNRESTRICTED,
    )?;
    println!("{}", svg);
    Ok(ExitCode::SUCCESS)
}
