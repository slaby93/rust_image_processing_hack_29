mod utils;
mod image_processing;
extern crate base64;
extern crate image;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn load_image(image_base_64: &str) -> String {
    utils::set_panic_hook();
    let decoded_base64: Vec<u8> = base64::decode(image_base_64).unwrap();
    let image: image::DynamicImage = image::load_from_memory_with_format(&decoded_base64, image::PNG)
        .ok()
        .expect("Opening image failed");
    
    let grayscaled_image = image_processing::pixellate(&image);

    let mut buf = Vec::new();

    grayscaled_image
        .write_to(&mut buf, image::ImageOutputFormat::PNG)
        .expect("Unable to write");
    
    let encoded_base64 = base64::encode(&buf);
    encoded_base64
}