use std::path::Path;
use image::DynamicImage;
use rand::Rng;

pub fn flip_horizontally(img: DynamicImage) -> DynamicImage {
    let filtered = img.fliph();
    filtered
}

pub fn flip_vertically(img: DynamicImage) -> DynamicImage {
    let filtered = img.flipv();
    filtered
}

pub fn invert(img: DynamicImage) -> DynamicImage {
    img.invert();
    img
}

pub fn grayscale(img: DynamicImage) -> DynamicImage {
    let filtered = img.grayscale();
    filtered
}