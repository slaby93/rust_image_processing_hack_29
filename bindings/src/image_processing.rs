extern crate rand;

use image::{DynamicImage, FilterType, GenericImageView, ImageBuffer, ImageRgba8, Pixel, Rgba};
use rand::distributions::{Distribution, Normal};
use std::f64;

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
    let (width, height) = img.dimensions();
    let original = img.to_rgba();
    let mut greyscale = img.grayscale().to_rgba();

    for x in 0..width {
        for y in 0..height {
            let original_pixel = original.get_pixel(x, y);
            let mut greyscale_pixel = *greyscale.get_pixel(x, y);
            greyscale_pixel[3] = original_pixel[3];
            greyscale.put_pixel(x, y, greyscale_pixel);
        }
    }

    ImageRgba8(greyscale)
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

pub fn rotate(img: &DynamicImage, degrees: f64) -> DynamicImage {
    if degrees > 90.0 || degrees < 0.0 {
        panic!("rotate only supports degrees from 0 to 90!")
    }

    let mut radians = degrees * f64::consts::PI / 180.0;
    let mut angle_cos = radians.cos();
    let mut angle_sin = radians.sin();
    
    let (width, height) = img.dimensions();
    let mut center_x = width as f64 / 2.0;
    let mut center_y = height as f64 / 2.0;

    // first, rotate the corners of the image to determine new image dimensions
    // rotate bottom left corner (x)
    let x_start = angle_cos * (-center_x) - angle_sin * (height as f64 - center_y) + center_x;
    // rotate top right corner (x)
    let x_end = angle_cos * (width as f64 - center_x) - angle_sin * (-center_y) + center_x;
    // rotate top left corner (y)
    let y_start = angle_sin * (-center_x) + angle_cos * (-center_y) + center_y;
    // rotate bottom right corner (y)
    let y_end = angle_sin * (width as f64 - center_x) + angle_cos * (height as f64 - center_y) + center_y;

    // move the results by (x_start.abs(), y_start.abs()) so coordinates start at 0
    let x_move = x_start.abs();
    let y_move = y_start.abs();
    let new_width = (x_move + x_end).ceil() as u32;
    let new_height = (y_move + y_end).ceil() as u32;
    center_x = new_width as f64 / 2.0 - x_move;
    center_y = new_height as f64 / 2.0 - y_move;
    let mut out = ImageBuffer::new(new_width, new_height);

    // Iterate over all of the pixels in the destination image - we're doing this "backwards" so we make sure
    // we set all of the pixels in the new image, as otherwise we might miss some (causing black dots in the
    // destination image) due to rounding issues.
    radians = (360.0 - degrees) * f64::consts::PI / 180.0;
    angle_cos = radians.cos();
    angle_sin = radians.sin();
    for y in 0..new_height {
        for x in 0..new_width {
            let old_x = (angle_cos * (x as f64 - center_x - x_move) - angle_sin * (y as f64 - center_y - y_move) + center_x).round() as u32;
            let old_y = (angle_sin * (x as f64 - center_x - x_move) + angle_cos * (y as f64 - center_y - y_move) + center_y).round() as u32;
            if old_x < width && old_y < height {
                let p = img.get_pixel(old_x, old_y);
                out.put_pixel(x, y, p);
            }
        }
    }

    ImageRgba8(out)
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
            let adjusted_watermark_pixel = image::Rgba([
                watermark_pixel[0],
                watermark_pixel[1],
                watermark_pixel[2],
                (watermark_pixel[3] as f32 * transparency) as u8,
            ]);
            let is_opaque = watermark_alpha != 0;

            if is_opaque {
                let mut new_pixel = *img_buffer.get_pixel(x, y);
                new_pixel.blend(&adjusted_watermark_pixel);
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
    let mut filtered = img.filter3x3(&kernel);
    for x in 0..3 {
        filtered = filtered.filter3x3(&kernel);
    }
    filtered
}

pub fn blur(img: &DynamicImage) -> DynamicImage {
    let mut kernel = [1.0f32, 2.0, 1.0, 2.0, 4.0, 2.0, 1.0, 2.0, 1.0];
    let scale = 16.0;
    let offset = 0.0;
    scale_kernel(&mut kernel, scale, offset);
    let mut filtered = img.filter3x3(&kernel);
    for x in 0..3 {
        filtered = filtered.filter3x3(&kernel);
    }
    filtered
}

pub fn scale_kernel(kernel: &mut [f32], scale: f32, offset: f32) {
    for value in kernel.iter_mut() {
        *value = *value / scale + offset;
    }
}
