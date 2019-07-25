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

pub fn best_fit_resize(img: DynamicImage, width: u32, height: u32) -> DynamicImage {
    // preserve the aspect ratio while scaling to the maximum possible size that fits within bounds specified by width and height.
    let filtered = img.resize(width, height, FilterType::Lanczos3);
    filtered
}


pub fn exact_resize(img: DynamicImage, width: u32, height: u32) -> DynamicImage {
    let filtered = img.resize_exact(width, height, FilterType::Lanczos3);
    filtered
}
