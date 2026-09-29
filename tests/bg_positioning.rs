//! Background-image positioning: geometry math + blog-hero rendering.

use cosy::schema::InputData;
use cosy::template::{load_template, process_template};
use cosy::text::ImagePolicy;
use std::path::PathBuf;

fn assets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates")
}

mod geom_unit {
    use cosy::template::bg_image_geom;

    #[test]
    fn defaults_reproduce_center_cover() {
        // same aspect as the canvas → fills exactly, no offset
        let (x, y, w, h) = bg_image_geom(1200.0, 675.0, 2400.0, 1350.0, 1.0, 0.5, 0.5);
        assert_eq!((x, y, w, h), (0.0, 0.0, 1200.0, 675.0));
    }

    #[test]
    fn tall_image_cover_crops_vertically() {
        // 1:2 image on a 16:9 canvas → cover scale = 1200/675 = 1.778
        let (x, y, w, h) = bg_image_geom(1200.0, 675.0, 675.0, 1350.0, 1.0, 0.5, 0.5);
        assert_eq!((x, y), (0.0, -862.5));
        assert_eq!((w, h), (1200.0, 2400.0));
    }

    #[test]
    fn zoom_doubles_display_size() {
        let (x, y, w, h) = bg_image_geom(1200.0, 675.0, 2400.0, 1350.0, 2.0, 0.5, 0.5);
        assert_eq!((x, y, w, h), (-600.0, -337.5, 2400.0, 1350.0));
    }

    #[test]
    fn focal_point_moves_the_window() {
        // image 2400×1350 at scale 2 → 2400×1350 displayed... s = 0.5 × 2 = 1.0,
        // so x = canvas_center − fx × 2400
        let (x, _, _, _) = bg_image_geom(1200.0, 675.0, 2400.0, 1350.0, 2.0, 0.0, 0.5);
        assert_eq!(x, 600.0); // fx=0: image left edge at canvas center
        let (_, y, _, _) = bg_image_geom(1200.0, 675.0, 2400.0, 1350.0, 2.0, 0.5, 1.0);
        assert_eq!(y, -1012.5); // fy=1: image bottom at canvas center
    }

    #[test]
    fn values_are_clamped() {
        // scale clamps to 0.1..=5.0, focal to 0..=1
        let (_, _, w, _) = bg_image_geom(1200.0, 675.0, 2400.0, 1350.0, 99.0, 0.5, 0.5);
        assert_eq!(w, 6000.0); // cover 0.5 × clamped scale 5 = 2.5 → 2400 × 2.5
        let (x, _, _, _) = bg_image_geom(1200.0, 675.0, 2400.0, 1350.0, 2.0, -3.0, 0.5);
        assert_eq!(x, 600.0); // fx clamped to 0 → x = 600 − 0 × 2400
    }
}

fn write_bg_png(dir: &std::path::Path) -> PathBuf {
    let img = image::RgbaImage::from_fn(1200, 600, |x, _| {
        if x < 600 {
            image::Rgba([255, 0, 0, 255])
        } else {
            image::Rgba([0, 0, 255, 255])
        }
    });
    let path = dir.join("bg.png");
    img.save(&path).expect("save fixture");
    path
}

#[test]
fn blog_hero_renders_positioned_background() {
    let dir = tempfile::tempdir().unwrap();
    let bg = write_bg_png(dir.path());

    let template = load_template("blog-hero").expect("template def");
    let template_dir = assets_dir().join("blog-hero");
    let brand = serde_json::json!({"brand_name": "t", "bg_image_scale": 3.0});
    let slide = serde_json::json!({"title": "T", "bg_image": bg.to_string_lossy()});

    let svg = process_template(
        &template,
        &template_dir,
        &brand,
        &slide,
        ImagePolicy::UNRESTRICTED,
    )
    .expect("render must succeed");

    assert!(
        svg.contains(r#"preserveAspectRatio="none""#),
        "positioned image must drop slice mode"
    );
    // img 1200×600 on 1200×675 canvas: cover = 1.125, ×3 → w 4050, h 2025,
    // x = 600 − 0.5 × 4050 = −1425 (f64 serializes with a trailing .0)
    assert!(svg.contains(r#"width="4050.0""#), "zoomed width missing");
    assert!(svg.contains(r#"height="2025.0""#), "zoomed height missing");
    assert!(svg.contains(r#"x="-1425.0""#), "centered offset missing");
}

#[test]
fn blog_hero_without_bg_has_no_geom() {
    let template = load_template("blog-hero").expect("template def");
    let template_dir = assets_dir().join("blog-hero");
    let data: InputData =
        serde_json::from_str(&std::fs::read_to_string(template_dir.join("defaults.json")).unwrap())
            .unwrap();
    let svg = process_template(
        &template,
        &template_dir,
        &data.brand,
        &data.slides[0],
        ImagePolicy::SECURE,
    )
    .expect("render must succeed");
    assert!(
        !svg.contains(r#"preserveAspectRatio="none""#),
        "no bg_image → no positioned image"
    );
}
