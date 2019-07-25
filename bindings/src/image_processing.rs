use image::DynamicImage;

pub fn flip_horizontally(img: DynamicImage) -> DynamicImage {
    let filtered = img.fliph();
    filtered
}

pub fn flip_vertically(img: DynamicImage) -> DynamicImage {
    let filtered = img.flipv();
    filtered
}

pub fn invert(img: DynamicImage) -> DynamicImage {
    let mut cloned_img = img.clone();
    cloned_img.invert();
    cloned_img
}

pub fn grayscale(img: DynamicImage) -> DynamicImage {
    let filtered = img.grayscale();
    filtered
}

pub fn pixellate(img: DynamicImage) -> DynamicImage {
    let rgbImage = img.to_rgb();

    img
}
