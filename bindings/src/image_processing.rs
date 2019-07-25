use image::{DynamicImage, FilterType, GenericImageView};

const PIXELLATE_SIZE: u32 = 8;

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
    let subsampled = img.resize(
        img.width() / PIXELLATE_SIZE,
        img.height() / PIXELLATE_SIZE,
        FilterType::Triangle,
    );
    subsampled.resize(img.width(), img.height(), FilterType::Nearest)
}

pub fn rotate_right(img: DynamicImage) -> DynamicImage {
    let filtered = img.rotate90();
    filtered
}


pub fn rotate_left(img: DynamicImage) -> DynamicImage {
    let filtered = img.rotate270();
    filtered
}
