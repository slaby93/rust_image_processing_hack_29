mod utils;
extern crate base64;
extern crate image;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(text: &str) {
    let text_to_display = format!("Your input: {}", text);
    alert(&text_to_display);
}

const IMAGE_PREFIX: &str = "data:image/png;base64,";

#[wasm_bindgen]
pub fn load_image(image_base_64: &str) {
    utils::set_panic_hook();
    let decoded_image = base64::decode("dGVzdA==").unwrap();
    alert(&format!("X: {:?}", String::from_utf8(decoded_image).unwrap()));
}