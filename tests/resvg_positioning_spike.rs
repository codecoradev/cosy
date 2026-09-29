//! Spike: verify resvg handles `preserveAspectRatio="none"` with explicit
//! x/y/width/height (the technique behind user-positioned background images)
//! including overflow clipping at the canvas edge.

use base64::Engine;

/// Build a 2×1 PNG: left pixel green, right pixel blue.
fn two_tone_png() -> String {
    let png_bytes = image::RgbaImage::from_fn(2, 1, |x, _| {
        if x == 0 {
            image::Rgba([0, 255, 0, 255]) // green
        } else {
            image::Rgba([0, 0, 255, 255]) // blue
        }
    });
    let mut out = Vec::new();
    image::DynamicImage::ImageRgba8(png_bytes)
        .write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)
        .unwrap();
    base64::engine::general_purpose::STANDARD.encode(out)
}

fn render(svg: &str) -> tiny_skia::Pixmap {
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg, &opt).expect("svg must parse");
    let mut pixmap = tiny_skia::Pixmap::new(100, 50).unwrap();
    resvg::render(
        &tree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    pixmap
}

fn pixel(pixmap: &tiny_skia::Pixmap, x: u32, y: u32) -> (u8, u8, u8) {
    let p = pixmap.pixel(x, y).unwrap();
    (p.red(), p.green(), p.blue())
}

#[test]
fn zoomed_image_with_offset_and_clipping() {
    let img_uri = format!("data:image/png;base64,{}", two_tone_png());

    // 100×50 canvas, red background. Image zoomed 2× (200×150), shifted so
    // its green half covers the left half of the canvas and the blue half
    // the right half; the rest overflows the canvas and must be clipped.
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="50" viewBox="0 0 100 50">
  <rect width="100" height="50" fill="red"/>
  <image href="{img}" x="-50" y="-50" width="200" height="150" preserveAspectRatio="none"/>
</svg>"##,
        img = img_uri,
    );
    let pixmap = render(&svg);

    // sample away from the pixel boundary — resvg smooths when scaling
    let (r, g, b) = pixel(&pixmap, 10, 25);
    assert!(
        g > 180 && b < 80 && r < 60,
        "left region must be green, got {r}/{g}/{b}"
    );
    let (r, g, b) = pixel(&pixmap, 90, 25);
    assert!(
        b > 180 && g < 80 && r < 60,
        "right region must be blue, got {r}/{g}/{b}"
    );
}

#[test]
fn focal_shift_changes_visible_region() {
    let img_uri = format!("data:image/png;base64,{}", two_tone_png());

    // Same zoom, shifted right so the image starts at x=50: pixels 0–49
    // fall outside the image (red background), 50–99 land on the green half.
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="50" viewBox="0 0 100 50">
  <rect width="100" height="50" fill="red"/>
  <image href="{img}" x="50" y="-50" width="200" height="150" preserveAspectRatio="none"/>
</svg>"##,
        img = img_uri,
    );
    let pixmap = render(&svg);

    let (r, g, b) = pixel(&pixmap, 10, 25);
    assert!(
        r > 200 && g < 60 && b < 60,
        "x<50 is outside the image: red background, got {r}/{g}/{b}"
    );
    let (r, g, b) = pixel(&pixmap, 90, 25);
    assert!(
        g > 180 && b < 80,
        "x in 50..100 lands on the green half, got {r}/{g}/{b}"
    );
}
