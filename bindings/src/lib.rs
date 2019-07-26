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
const PIXELLATE_SIZE: u32 = 8;


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
    img = img.flipv();
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn invert(img: String) -> String {
    let mut img = base_64_to_img(&img);
    img.invert();
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn grayscale(img: String) -> String {
    let img = base_64_to_img(&img);
    img_to_base_64(&img.grayscale())
}

#[wasm_bindgen]
pub fn pixellate(img: String) -> String {
    let img = base_64_to_img(&img);
    let subsampled = img.resize(
        img.width() / PIXELLATE_SIZE,
        img.height() / PIXELLATE_SIZE,
        FilterType::Triangle,
    );
    subsampled.resize(img.width(), img.height(), FilterType::Nearest);
    img_to_base_64(&subsampled)
}

#[wasm_bindgen]
pub fn rotate_right(img: String) -> String {
    let img = base_64_to_img(&img);
    img_to_base_64(&img.rotate90())
}

#[wasm_bindgen]
pub fn rotate_left(img: String) -> String {
    let img = base_64_to_img(&img);
    img_to_base_64(&img.rotate270())
}

#[wasm_bindgen]
pub fn best_fit_resize(img: String, width: u32, height: u32) -> String {
    let mut img = base_64_to_img(&img);
    // preserve the aspect ratio while scaling to the maximum possible size that fits within bounds specified by width and height.
    img = img.resize(width, height, FilterType::Lanczos3);
    img_to_base_64(&img)
}

#[wasm_bindgen]
pub fn exact_resize(img: String, width: u32, height: u32) -> String {
    let mut img = base_64_to_img(&img);
    img = img.resize_exact(width, height, FilterType::Lanczos3);
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

    // adds watermark in top left corner
    // original source image should be bigger than logo, otherwise it will crash
    // with transparency 0.0 the logo will be fully opaque
    // with transparency 1.0 the logo will be invisible
    let (width, height) = img_watermark.dimensions();
    let mut img_buffer = img.to_rgba();
    let watermark_buffer = img_watermark.to_rgba();

    for x in 0..width {
        for y in 0..height {
            let watermark_pixel = watermark_buffer.get_pixel(x, y);

            let watermark_alpha = watermark_pixel[3];
            let is_opaque = watermark_alpha != 0;

            if is_opaque {
                let mut new_pixel = *img_buffer.get_pixel(x, y);

                for channel in 0..3 {
                    new_pixel[channel] = (
                        (new_pixel[channel] as f32) * transparency + (watermark_pixel[channel] as f32) * (1.0 - transparency)
                    ) as u8;
                }

                img_buffer.put_pixel(x, y, new_pixel);
            }
        }
    }
    img_to_base_64(&ImageRgba8(img_buffer))
}
