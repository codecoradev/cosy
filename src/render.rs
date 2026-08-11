//! Core rendering pipeline: JSON data → minijinja → SVG → resvg → PNG

use std::path::Path;
use crate::schema::{InputData, TemplateDef};

/// Render a template with data to output file(s)
pub fn render_template(
    template_name: &str,
    data_path: &str,
    output: &Path,
    scale: f32,
) -> anyhow::Result<()> {
    // 1. Load template
    let template = crate::template::load_template(template_name)?;
    log::info!("Loaded template: {} ({}x{})",
        template.name, template.dimensions.width, template.dimensions.height);

    // 2. Load input data
    let data = InputData::from_file(data_path)?;
    log::info!("Loaded data: {} slide(s)", data.slides.len());

    // 3. For each slide, render
    if data.is_single_slide() || output.extension().is_some() {
        // Single image output
        let png = render_slide(&template, &data, 0)?;
        std::fs::write(output, &png)?;
        log::info!("Written: {:?}", output);
    } else {
        // Multi-slide output to directory
        std::fs::create_dir_all(output)?;
        let width = (template.dimensions.width as f32 * scale) as u32;
        let height = (template.dimensions.height as f32 * scale) as u32;

        for (i, _slide) in data.slides.iter().enumerate() {
            let png = render_slide(&template, &data, i)?;
            let filename = format!("{:02}.png", i + 1);
            let path = output.join(&filename);
            std::fs::write(&path, &png)?;
            log::info!("Written: {:?}", path);
        }
        log::info!("Rendered {} slide(s) to {:?}", data.slides.len(), output);
    }

    Ok(())
}

/// Render a single slide to PNG bytes
fn render_slide(
    template: &TemplateDef,
    data: &InputData,
    slide_index: usize,
) -> anyhow::Result<Vec<u8>> {
    // 1. Build minijinja context from brand fields + slide fields
    let slide_data = &data.slides[slide_index];

    // 2. Pre-process text wrapping for long text fields
    // 3. Apply minijinja token replacement to SVG template
    let svg_string = crate::template::apply_template(template, &data.brand, slide_data)?;

    // 4. Parse SVG via usvg
    let opts = usvg::Options::default();
    let tree = usvg::Tree::from_str(&svg_string, &opts)?;

    // 5. Render via resvg → tiny-skia
    let width = template.dimensions.width;
    let height = template.dimensions.height;
    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| anyhow::anyhow!("Failed to create pixmap {}x{}", width, height))?;

    resvg::render(&tree, tiny_skia::Transform::identity(), &mut pixmap.as_mut());

    // 6. Encode to PNG
    Ok(pixmap.encode_png()?)
}
