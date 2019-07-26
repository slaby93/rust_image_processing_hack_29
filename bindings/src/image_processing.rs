extern crate rand;

use image::{DynamicImage, FilterType, GenericImageView, ImageBuffer, ImageRgba8, Rgba};
use rand::distributions::{Distribution, Normal};

const PIXELLATE_SIZE: u32 = 8;

pub fn flip_horizontally(img: &DynamicImage) -> DynamicImage {
    let filtered = img.fliph();
    filtered
}

pub fn flip_vertically(img: &DynamicImage) -> DynamicImage {
    let filtered = img.flipv();
    filtered
}

pub fn invert(img: &DynamicImage) -> DynamicImage {
    let mut cloned_img = img.clone();
    cloned_img.invert();
    cloned_img
}

pub fn grayscale(img: &DynamicImage) -> DynamicImage {
    let filtered = img.grayscale();
    filtered
}

pub fn pixellate(img: &DynamicImage) -> DynamicImage {
    let width = img.width();
    let height = img.height();
    let subsampled = img.resize_exact(
        width / PIXELLATE_SIZE,
        height / PIXELLATE_SIZE,
        FilterType::Triangle,
    );
    subsampled.resize_exact(width, height, FilterType::Nearest)
}

pub fn rotate_right(img: &DynamicImage) -> DynamicImage {
    let filtered = img.rotate90();
    filtered
}

pub fn rotate_left(img: &DynamicImage) -> DynamicImage {
    let filtered = img.rotate270();
    filtered
}

pub fn best_fit_resize(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    // preserve the aspect ratio while scaling to the maximum possible size that fits within bounds specified by width and height.
    let filtered = img.resize(width, height, FilterType::Lanczos3);
    filtered
}

pub fn exact_resize(img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
    let filtered = img.resize_exact(width, height, FilterType::Lanczos3);
    filtered
}

pub fn add_watermark(
    original_img: &DynamicImage,
    watermark_img: &DynamicImage,
    transparency: f32,
) -> DynamicImage {
    // adds watermark in top left corner
    // original source image should be bigger than logo, otherwise it will crash
    // with transparency 0.0 the logo will be fully opaque
    // with transparency 1.0 the logo will be invisible
    let (width, height) = watermark_img.dimensions();
    let mut img_buffer = original_img.to_rgba();
    let watermark_buffer = watermark_img.to_rgba();

    for x in 0..width {
        for y in 0..height {
            let watermark_pixel = watermark_buffer.get_pixel(x, y);

            let watermark_alpha = watermark_pixel[3];
            let is_opaque = watermark_alpha != 0;

            if is_opaque {
                let mut new_pixel = *img_buffer.get_pixel(x, y);

                for channel in 0..3 {
                    new_pixel[channel] = ((new_pixel[channel] as f32) * transparency
                        + (watermark_pixel[channel] as f32) * (1.0 - transparency))
                        as u8;
                }

                img_buffer.put_pixel(x, y, new_pixel);
            }
        }
    }
    ImageRgba8(img_buffer)
}

pub fn detect_edges(img: &DynamicImage) -> DynamicImage {
    let kernel = [-1.0f32, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];
    let filtered = img.filter3x3(&kernel);
    filtered
}

pub fn add_noise(img: &DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut rng = rand::thread_rng();
    let normal = Normal::new(50.0, 50.0);
    let mut noisy = img.brighten(-80).to_rgba();
    let img = img.to_rgba();
    for x in 0..width {
        for y in 0..height {
            let offset = normal.sample(&mut rng) as u8;
            let mut px = *img.get_pixel(x, y);
            noisy.put_pixel(x, y, px);
        }
    }
    ImageRgba8(noisy)
}

pub fn enhance_edges(img: &DynamicImage) -> DynamicImage {
    let mut kernel = [-1.0f32, -1.0, -1.0, -1.0, 10.0, -1.0, -1.0, -1.0, -1.0];
    let scale = 2.0;
    let offset = 0.0;
    scale_kernel(&mut kernel, scale, offset);
    let filtered = img.filter3x3(&kernel);
    filtered
}

pub fn detail(img: &DynamicImage) -> DynamicImage {
    let mut kernel = [0.0f32, -1.0, 0.0, -1.0, 10.0, -1.0, 0.0, -1.0, 0.0];
    let scale = 6.0;
    let offset = 0.0;
    scale_kernel(&mut kernel, scale, offset);
    let filtered = img.filter3x3(&kernel);
    filtered
}

pub fn emboss(img: &DynamicImage) -> DynamicImage {
    let mut kernel = [-1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    let scale = 1.0;
    let offset = 128.0;
    scale_kernel(&mut kernel, scale, offset);
    let filtered = img.filter3x3(&kernel);
    filtered
}

pub fn smoothen(img: &DynamicImage) -> DynamicImage {
    let mut kernel = [1.0f32, 1.0, 1.0, 1.0, 5.0, 1.0, 1.0, 1.0, 1.0];
    let scale = 13.0;
    let offset = 0.0;
    scale_kernel(&mut kernel, scale, offset);
    let filtered = img.filter3x3(&kernel);
    filtered
}

pub fn contour(img: &DynamicImage) -> DynamicImage {
    let mut kernel = [-1.0f32, -1.0, -1.0, -1.0, 8.0, -1.0, -1.0, -1.0, -1.0];
    let scale = 1.0;
    let offset = 255.0;
    scale_kernel(&mut kernel, scale, offset);
    let filtered = img.filter3x3(&kernel);
    filtered
}

pub fn scale_kernel(kernel: &mut [f32], scale: f32, offset: f32) {
    for value in kernel.iter_mut() {
        *value = *value / scale + offset;
    }
}
