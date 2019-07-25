use image::{GenericImageView, DynamicImage, ImageBuffer, Rgba, ImageRgba8};

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


pub fn add_watermark(
    original_img: DynamicImage,
    watermark_img: DynamicImage,
    transparency: f32
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
                    new_pixel[channel] = (
                        (new_pixel[channel] as f32) * transparency + (watermark_pixel[channel] as f32) * (1.0 - transparency)
                    ) as u8;
                }

                img_buffer.put_pixel(x, y, new_pixel);
            }
        }
    }
    ImageRgba8(img_buffer)
}
