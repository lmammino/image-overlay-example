use std::env;

use image::{imageops::overlay, io::Reader};

fn main() {
    let assets_path = env::current_dir().unwrap().join("assets");
    let output_path = env::current_dir().unwrap().join("output.png");
    let base_img_path = assets_path.join("base.jpg");
    let overlay_img_path = assets_path.join("overlay.png");
    let base_img = Reader::open(base_img_path).unwrap().decode().unwrap();
    let overlay_img = Reader::open(overlay_img_path).unwrap().decode().unwrap();
    let mut canvas = base_img.resize(1200, 1200, image::imageops::FilterType::Lanczos3);
    overlay(&mut canvas, &overlay_img, 0, 0);

    canvas.save(output_path).expect("Failed to save image.");
}
