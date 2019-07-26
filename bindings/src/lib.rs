mod utils;
extern crate base64;
extern crate image;
mod image_processing;
use image::DynamicImage;
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

const WEB_BASE64_PREFIX_PNG: &str = "data:image/png;base64,";
const WEB_BASE64_PREFIX_JPEG: &str = "data:image/jpeg;base64,";

#[wasm_bindgen]
pub fn load_image(image_base_64: &str) -> String {
    utils::set_panic_hook();
    let (image_base_64_clear, prefix, extension) = split_base64_prefix(image_base_64);
    let decoded_base64: Vec<u8> = base64::decode(image_base_64_clear).unwrap();
    let image: image::DynamicImage = image::load_from_memory_with_format(&decoded_base64, extension)
        .ok()
        .expect("Opening image failed");

    let grayscaled_image = image_processing::pixellate(&image);


    let mut buf = Vec::new();

    grayscaled_image
        .write_to(&mut buf, image::ImageOutputFormat::PNG)
        .expect("Unable to write");

    let encoded_base64 = base64::encode(&buf);
    format!("{}{}", String::from(prefix), String::from(encoded_base64))
}

pub fn split_base64_prefix(base64Image: &str) -> (&str, &str, image::ImageFormat) {
    if(base64Image.contains(WEB_BASE64_PREFIX_PNG)) {
        let base64 = base64Image.split(WEB_BASE64_PREFIX_PNG).collect::<Vec<&str>>()[1];
        return (base64, WEB_BASE64_PREFIX_PNG, image::PNG);
    } else if (base64Image.contains(WEB_BASE64_PREFIX_JPEG)) {
        let base64 = base64Image.split(WEB_BASE64_PREFIX_JPEG).collect::<Vec<&str>>()[1];
        return (base64, WEB_BASE64_PREFIX_JPEG, image::JPEG);
    } else {
        return ("", "", image::PNG);
    }
}

fn base_64_to_img(image_base_64: &str) -> DynamicImage {
    let decoded_base64: Vec<u8> = base64::decode(image_base_64).unwrap();
    let image: image::DynamicImage = image::load_from_memory_with_format(&decoded_base64, image::PNG)
        .ok()
        .expect("Opening image failed");
    image
}

fn img_to_base_64(img: &DynamicImage) -> String {
    let mut buf = Vec::new();
    img
        .write_to(&mut buf, image::ImageOutputFormat::PNG)
        .expect("Unable to write");

    let encoded_base64 = base64::encode(&buf);
    encoded_base64
}

#[wasm_bindgen]
pub struct Image {
    img: DynamicImage,
}

#[wasm_bindgen]
impl Image {

    pub fn set_image(&self, image_base_64: &str) -> Image {
        let img = base_64_to_img(image_base_64);
        Image{
            img,
        }
    }

    pub fn flip_h(&mut self) -> String {
        self.img = image_processing::flip_horizontally(&self.img);
        img_to_base_64(&self.img)
    }

}
