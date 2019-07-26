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
    img_watermark: String,
    transparency: f32
) -> String {
    let mut img = base_64_to_img(&img);
    let mut img_watermark = base_64_to_img(&img_watermark);

    img = image_processing::add_watermark(&img, &img_watermark, transparency);

    img_to_base_64(&img)
}
