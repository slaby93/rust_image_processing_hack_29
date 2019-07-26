mod utils;
mod image_processing;
extern crate base64;
extern crate image;
use wasm_bindgen::prelude::*;
use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba, ImageRgba8, FilterType};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

const WEB_BASE64_PREFIX_PNG: &str = "data:image/png;base64,";
const WEB_BASE64_PREFIX_JPEG: &str = "data:image/jpeg;base64,";


fn base_64_to_img(image_base_64: &str) -> DynamicImage {
    let decoded_base64: Vec<u8> = base64::decode(image_base_64).unwrap();
    let img: image::DynamicImage = image::load_from_memory_with_format(&decoded_base64, image::PNG)
        .ok()
        .expect("Opening image failed");
    img
}

fn img_to_base_64(img: &DynamicImage) -> String {
    let mut buf = Vec::new();
    img.write_to(&mut buf, image::ImageOutputFormat::PNG)
        .expect("Unable to write");

    let encoded_base64 = base64::encode(&buf);
    encoded_base64
}

#[wasm_bindgen]
pub fn flip_horizontally(img: String) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::flip_horizontally(&img);
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn flip_vertically(img: String) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::flip_vertically(&img);
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn invert(img: String) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::invert(&img);
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn grayscale(img: String) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::grayscale(&img);
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn pixellate(img: String) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::pixellate(&img);
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn rotate_right(img: String) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::rotate_right(&img);
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn rotate_left(img: String) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::rotate_left(&img);
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn best_fit_resize(img: String, width: u32, height: u32) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::best_fit_resize(&img, width, height);
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn exact_resize(img: String, width: u32, height: u32) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::exact_resize(&img, width, height);
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn add_watermark(
    img: String,
) -> String {
    let mut img = base_64_to_img(&img);
    let img_watermark = "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";
    let img_watermark = base_64_to_img(&img_watermark);
    let transparency = 0.5;

    img = image_processing::add_watermark(&img, &img_watermark, transparency);

    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn enhance_edges(img: String) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::enhance_edges(&img);
    img_to_base_64(&img)

}

#[wasm_bindgen]
pub fn rotate(img: String, angle: f64,) -> String {
    let mut img = base_64_to_img(&img);
    img = image_processing::rotate(&img, angle);
    img_to_base_64(&img)

}