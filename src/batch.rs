//! CSV batch rendering: one CSV row → one render, in parallel.
//!
//! Design (per issue #96, Robolly Datasets parity):
//! - CSV header = JSON keys; each row = one single-slide input.
//! - Brand defaults come from the template's `defaults.json` `brand` object.
//! - Row → slide mapping:
//!   - default: every column becomes a slide field (string values); fields
//!     the schema declares as `number`/`boolean` are coerced so validation
//!     and template arithmetic see real types.
//!   - a `_data` column overrides the flat mapping entirely: its cell is a
//!     complete JSON slide object, an array of slides, or a full
//!     `{"brand": …, "slides": […]}` document (multi-slide renders write a
//!     `{stem}_slides/NN.ext` subdirectory).
//! - Optional `_filename` column sets the output file stem (sanitized);
//!   otherwise `{row_index}_{slug-of-first-text-field}`.
//! - Rows render in parallel (rayon). `fail_fast` stops *unscheduled* rows
//!   after the first failure; already-started rows complete.
//! - Reserved columns (`_filename`, `_data`) never reach the template.

use crate::format::OutputFormat;
use crate::schema::{FieldType, InputData, TemplateDef};
use rayon::prelude::*;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// One row's render outcome.
#[derive(Debug)]
pub struct RowResult {
    pub row_index: usize,
    pub files: Vec<String>,
    pub error: Option<String>,
}

/// Summary of a batch run (machine-serializable via `--json-output`).
#[derive(Debug, serde::Serialize)]
pub struct BatchResult {
    pub template: String,
    pub total_rows: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub files: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<String>,
    pub render_time_ms: u128,
}

/// One row's prepared render input.
struct RowJob {
    row_index: usize,
    /// Output file stem (no extension), relative to `out_dir`.
    stem: String,
    /// Complete input data for this row.
    data: InputData,
    /// Rows with >1 slide (via `_data`) render into a subdirectory.
    is_multi: bool,
    /// Row-level preparation error (invalid `_data` JSON, etc.).
    prep_error: Option<String>,
}

/// Render a CSV dataset: each row becomes one render of the template.
#[allow(clippy::too_many_arguments)]
pub fn render_csv(
    template_name: &str,
    csv_path: &Path,
    out_dir: &Path,
    scale: f32,
    font_dir: Option<&Path>,
    image_policy: crate::text::ImagePolicy,
    format: OutputFormat,
    fail_fast: bool,
) -> anyhow::Result<BatchResult> {
    let start = std::time::Instant::now();

    // 1. Load template + brand defaults (once, shared read-only)
    let template = crate::template::load_template(template_name)?;
    let template_dir = crate::template::find_template_dir_for(template_name)?;
    let font_db = crate::render::build_font_db(font_dir)?;
    let brand_defaults = load_brand_defaults(&template_dir)?;

    // 2. Parse CSV (RFC4180: quoted cells, escaped quotes, CRLF)
    let rows = parse_csv(std::fs::read_to_string(csv_path)?)?;
    let (header, data_rows) = match rows.split_first() {
        Some((h, d)) => (h, d),
        None => anyhow::bail!("CSV file is empty: {}", csv_path.display()),
    };

    // 3. Prepare jobs sequentially (cheap string work)
    let jobs: Vec<RowJob> = data_rows
        .iter()
        .enumerate()
        .map(|(i, row)| build_row_job(&template, header, row, i + 1, &brand_defaults))
        .collect();

    std::fs::create_dir_all(out_dir)?;

    // 4. Render in parallel. fail_fast = once one row fails, rows that
    //    haven't started yet report "skipped" instead of rendering.
    let stop = AtomicBool::new(false);
    let results: Vec<RowResult> = jobs
        .par_iter()
        .map(|job| {
            if fail_fast && stop.load(Ordering::Relaxed) {
                return RowResult {
                    row_index: job.row_index,
                    files: vec![],
                    error: Some("skipped (fail-fast)".into()),
                };
            }
            let r = render_one_row(
                &template,
                &template_dir,
                &font_db,
                image_policy,
                scale,
                format,
                out_dir,
                job,
            );
            if r.error.is_some() {
                stop.store(true, Ordering::Relaxed);
            }
            r
        })
        .collect();

    // 5. Summarize (file order preserved by collect)
    let mut files = Vec::new();
    let mut errors = Vec::new();
    let mut succeeded = 0usize;
    let mut failed = 0usize;
    for r in results {
        match r.error {
            None => {
                succeeded += 1;
                files.extend(r.files);
            }
            Some(e) => {
                failed += 1;
                errors.push(format!("row {}: {}", r.row_index, e));
            }
        }
    }

    log::info!(
        "Batch: {}/{} rows OK in {}ms",
        succeeded,
        succeeded + failed,
        start.elapsed().as_millis()
    );

    Ok(BatchResult {
        template: template_name.to_string(),
        total_rows: succeeded + failed,
        succeeded,
        failed,
        files,
        errors,
        render_time_ms: start.elapsed().as_millis(),
    })
}

/// Load the `brand` object out of the template's defaults.json (missing
/// file or missing brand → empty object).
fn load_brand_defaults(template_dir: &Path) -> anyhow::Result<serde_json::Value> {
    let path = template_dir.join("defaults.json");
    if !path.exists() {
        return Ok(serde_json::json!({}));
    }
    let parsed: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
    Ok(parsed
        .get("brand")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({})))
}

/// Map one CSV row to a render job.
fn build_row_job(
    template: &TemplateDef,
    header: &[String],
    row: &[String],
    row_index: usize,
    brand_defaults: &serde_json::Value,
) -> RowJob {
    // Pad short rows with empty cells (trailing commas are common).
    if row.len() > header.len() {
        // Extra cells usually mean unquoted commas inside a cell (e.g.
        // raw JSON in a _data column). Error clearly instead of silently
        // truncating.
        return RowJob {
            row_index,
            stem: format!("{:04}_row", row_index),
            data: empty_input(),
            is_multi: false,
            prep_error: Some(format!(
                "row has {} cells but header has {} — quote cells that contain commas",
                row.len(),
                header.len()
            )),
        };
    }
    let mut cells: Vec<String> = row.to_vec();
    cells.resize(header.len(), String::new());

    let mut record = serde_json::Map::new();
    for (k, v) in header.iter().zip(cells.iter()) {
        if k.is_empty() {
            continue;
        }
        record.insert(k.clone(), serde_json::Value::String(v.clone()));
    }

    // Stem always carries the row index: row numbers are unique per batch,
    // so duplicate `_filename` values across rows can never silently
    // overwrite each other (rows render in parallel — same-path concurrent
    // writes would race).
    let stem = format!(
        "{:04}_{}",
        row_index,
        record
            .get("_filename")
            .and_then(|v| v.as_str())
            .map(sanitize_filename)
            .filter(|s| !s.is_empty() && s != "row")
            .unwrap_or_else(|| derive_slug(&record))
    );

    // _data: full JSON slide override (multi-slide capable)
    if let Some(raw) = record.get("_data").and_then(|v| v.as_str()) {
        if !raw.trim().is_empty() {
            return match serde_json::from_str::<serde_json::Value>(raw) {
                Err(e) => RowJob {
                    row_index,
                    stem,
                    data: empty_input(),
                    is_multi: false,
                    prep_error: Some(format!("invalid _data JSON: {e}")),
                },
                Ok(parsed) => {
                    let (brand, slides) = split_brand_and_slides(parsed);
                    if slides.is_empty() {
                        RowJob {
                            row_index,
                            stem,
                            data: empty_input(),
                            is_multi: false,
                            prep_error: Some(
                                "_data produced no slides (empty array or null)".into(),
                            ),
                        }
                    } else {
                        let brand = if brand.is_object()
                            && brand.as_object().map(|o| !o.is_empty()).unwrap_or(false)
                        {
                            brand
                        } else {
                            brand_defaults.clone()
                        };
                        let is_multi = slides.len() > 1;
                        RowJob {
                            row_index,
                            stem,
                            data: InputData { brand, slides },
                            is_multi,
                            prep_error: None,
                        }
                    }
                }
            };
        }
    }

    // Flat mapping: type-coerce columns per the template schema.
    for (name, spec) in &template.slide_fields {
        if let Some(v) = record.get(name).and_then(|v| v.as_str()) {
            let coerced = match spec.field_type {
                FieldType::Number => v
                    .parse::<i64>()
                    .map(|n| serde_json::json!(n))
                    .or_else(|_| v.parse::<f64>().map(|n| serde_json::json!(n)))
                    .unwrap_or_else(|_| serde_json::json!(v)),
                FieldType::Boolean => match v {
                    "true" | "1" | "yes" => serde_json::json!(true),
                    "false" | "0" | "no" => serde_json::json!(false),
                    _ => serde_json::json!(v),
                },
                _ => serde_json::json!(v),
            };
            record.insert(name.clone(), coerced);
        }
    }

    // Reserved columns never reach the template.
    record.remove("_filename");
    record.remove("_data");

    RowJob {
        row_index,
        stem,
        data: InputData {
            brand: brand_defaults.clone(),
            slides: vec![serde_json::Value::Object(record)],
        },
        is_multi: false,
        prep_error: None,
    }
}

fn empty_input() -> InputData {
    InputData {
        brand: serde_json::json!({}),
        slides: vec![],
    }
}

/// Render one row; errors are captured per-row, never fatal to the batch.
#[allow(clippy::too_many_arguments)]
fn render_one_row(
    template: &TemplateDef,
    template_dir: &Path,
    font_db: &usvg::fontdb::Database,
    image_policy: crate::text::ImagePolicy,
    scale: f32,
    format: OutputFormat,
    out_dir: &Path,
    job: &RowJob,
) -> RowResult {
    if let Some(msg) = &job.prep_error {
        return RowResult {
            row_index: job.row_index,
            files: vec![],
            error: Some(msg.clone()),
        };
    }

    let outcome = (|| -> anyhow::Result<Vec<String>> {
        // Validate this row before rendering (same rules as CLI render).
        let errors = crate::template::validate_input(template, &job.data);
        if !errors.is_empty() {
            anyhow::bail!("validation failed: {}", errors.join("; "));
        }

        if job.is_multi {
            // One subdirectory per row, slides as NN.<ext>
            let dir = out_dir.join(format!("{}_slides", job.stem));
            std::fs::create_dir_all(&dir)?;
            let mut out = Vec::new();
            for i in 0..job.data.slides.len() {
                let pixels = crate::render::render_slide_to_pixels(
                    template,
                    template_dir,
                    &job.data,
                    i,
                    scale,
                    font_db,
                    image_policy,
                )?;
                let bytes = encode_pixels(template, scale, format, &pixels)?;
                let path = dir.join(format!("{:02}.{}", i + 1, format.extension()));
                std::fs::write(&path, &bytes)?;
                out.push(path.to_string_lossy().to_string());
            }
            return Ok(out);
        }

        // Single image per row
        let pixels = crate::render::render_slide_to_pixels(
            template,
            template_dir,
            &job.data,
            0,
            scale,
            font_db,
            image_policy,
        )?;
        let bytes = encode_pixels(template, scale, format, &pixels)?;
        let path = out_dir.join(format!("{}.{}", job.stem, format.extension()));
        std::fs::write(&path, &bytes)?;
        Ok(vec![path.to_string_lossy().to_string()])
    })();

    match outcome {
        Ok(files) => RowResult {
            row_index: job.row_index,
            files,
            error: None,
        },
        Err(e) => RowResult {
            row_index: job.row_index,
            files: vec![],
            error: Some(format!("{e:#}")),
        },
    }
}

fn encode_pixels(
    template: &TemplateDef,
    scale: f32,
    format: OutputFormat,
    pixels: &[u8],
) -> anyhow::Result<Vec<u8>> {
    let w = (template.dimensions.width as f32 * scale).round() as u32;
    let h = (template.dimensions.height as f32 * scale).round() as u32;
    format.encode(pixels, w, h)
}

/// Split a `_data` document into (brand, slides). Accepts:
/// - `{"brand": {...}, "slides": [...]}` → both parts
/// - `{...}` (bare slide) → defaults brand + one slide
/// - `[...]` (array of slides) → defaults brand + slides
fn split_brand_and_slides(
    parsed: serde_json::Value,
) -> (serde_json::Value, Vec<serde_json::Value>) {
    match parsed {
        serde_json::Value::Array(items) => (serde_json::json!({}), items),
        serde_json::Value::Object(obj) => {
            if let (Some(brand), Some(slides)) = (obj.get("brand"), obj.get("slides")) {
                if let Some(arr) = slides.as_array() {
                    return (brand.clone(), arr.clone());
                }
            }
            (serde_json::json!({}), vec![serde_json::Value::Object(obj)])
        }
        _ => (serde_json::json!({}), vec![]),
    }
}

/// Sanitize a caller-provided filename stem: keep alphanumerics, dash,
/// underscore; collapse the rest (including dots — no traversal, no
/// embedded extensions); trim; cap length.
fn sanitize_filename(s: &str) -> String {
    let mapped: String = s
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_') {
                c
            } else {
                '-'
            }
        })
        .collect();
    // Collapse consecutive dashes, trim leading/trailing ones.
    let mut collapsed = String::with_capacity(mapped.len());
    let mut prev_dash = false;
    for c in mapped.chars() {
        if c == '-' {
            if !prev_dash {
                collapsed.push(c);
            }
            prev_dash = true;
        } else {
            collapsed.push(c);
            prev_dash = false;
        }
    }
    let trimmed = collapsed.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "row".into()
    } else {
        trimmed.chars().take(80).collect()
    }
}

/// Derive a slug from the first non-reserved string value in the record.
fn derive_slug(record: &serde_json::Map<String, serde_json::Value>) -> String {
    record
        .iter()
        .filter(|(k, _)| !k.starts_with('_'))
        .find_map(|(_, v)| v.as_str())
        .map(sanitize_filename)
        .unwrap_or_else(|| "row".into())
}

/// Minimal RFC4180 CSV parser: quoted cells, escaped quotes (`""`),
/// comma separators, CR/LF/CRLF record ends.
pub fn parse_csv(input: String) -> anyhow::Result<Vec<Vec<String>>> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut cell = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if in_quotes {
            match c {
                '"' => {
                    if chars.peek() == Some(&'"') {
                        chars.next();
                        cell.push('"');
                    } else {
                        in_quotes = false;
                    }
                }
                _ => cell.push(c),
            }
        } else {
            match c {
                '"' if cell.is_empty() => in_quotes = true,
                ',' => {
                    row.push(std::mem::take(&mut cell));
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                    row.push(std::mem::take(&mut cell));
                    rows.push(std::mem::take(&mut row));
                }
                '\n' => {
                    row.push(std::mem::take(&mut cell));
                    rows.push(std::mem::take(&mut row));
                }
                _ => cell.push(c),
            }
        }
    }
    // Final cell/row without trailing newline
    if !cell.is_empty() || !row.is_empty() {
        row.push(cell);
        rows.push(row);
    }
    // Drop a trailing fully-empty row
    if let Some(last) = rows.last() {
        if last.len() == 1 && last[0].is_empty() {
            rows.pop();
        }
    }
    Ok(rows)
}

#[cfg(test)]
mod batch_tests {
    use super::*;

    #[test]
    fn parse_csv_basic() {
        let rows = parse_csv("a,b,c\n1,2,3\n".into()).unwrap();
        assert_eq!(rows, vec![vec!["a", "b", "c"], vec!["1", "2", "3"]]);
    }

    #[test]
    fn parse_csv_quoted_with_comma_and_quotes() {
        let rows = parse_csv("name,quote\n\"Doe, Jane\",\"\"\"Hi\"\" she said\"\n".into()).unwrap();
        assert_eq!(rows[1], vec!["Doe, Jane", "\"Hi\" she said"]);
    }

    #[test]
    fn parse_csv_crlf_and_empty_cells() {
        let rows = parse_csv("a,b\r\nx,\r\n".into()).unwrap();
        assert_eq!(rows, vec![vec!["a", "b"], vec!["x", ""]]);
    }

    #[test]
    fn parse_csv_newline_inside_quotes() {
        let rows = parse_csv("a,b\n\"line1\nline2\",y\n".into()).unwrap();
        assert_eq!(rows[1], vec!["line1\nline2", "y"]);
    }

    #[test]
    fn parse_csv_no_trailing_newline() {
        let rows = parse_csv("a,b\n1,2".into()).unwrap();
        assert_eq!(rows, vec![vec!["a", "b"], vec!["1", "2"]]);
    }

    #[test]
    fn sanitize_filename_strips_unsafe() {
        assert_eq!(sanitize_filename("hello world/x"), "hello-world-x");
        assert_eq!(sanitize_filename("../../etc"), "etc");
        assert_eq!(sanitize_filename("   "), "row");
        assert_eq!(sanitize_filename("café-bar"), "caf-bar");
    }
}
