use std::path::Path;
use image::DynamicImage;

pub fn flip_horizontally(path: &str) -> DynamicImage {
    let img = image::open(&Path::new(path)).ok().expect("Opening image failed");
    let filtered = img.fliph();
    filtered
}

pub fn flip_vertically(path: &str) -> DynamicImage {
    let img = image::open(&Path::new(path)).ok().expect("Opening image failed");
    let filtered = img.flipv();
    filtered
}

pub fn invert(path: &str) -> DynamicImage {
    let mut img = image::open(&Path::new(path)).ok().expect("Opening image failed");
    img.invert();
    img
}

pub fn grayscale(path: &str) -> DynamicImage {
    let img = image::open(&Path::new(path)).ok().expect("Opening image failed");
    let filtered = img.grayscale();
    filtered
}
