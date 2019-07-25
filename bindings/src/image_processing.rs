use std::path::Path;
use image::DynamicImage;
use rand::Rng;

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

pub fn parse_img(path: &str) {
    // Open the img at the specified file path, get its dimensions, and randomly reassign its RBGA values
    // into a new img. 
    // The edited data structure is an ImageBuffer linked below.
    // 
    // https://docs.rs/image/0.14.0/image/struct.ImageBuffer.html

    // TODO: Add error handling
    let mut img = image::open(path).unwrap().to_rgba();
    let mut rng = rand::thread_rng();

    let (x, y) = img.dimensions();

    for x_coordinate in 0..x {
        for y_coordinate in 0..y {
            let mut pixel = *img.get_pixel(x_coordinate, y_coordinate);

            pixel[0] = rng.gen_range(0, 255);
            pixel[1] = rng.gen_range(0, 255);
            pixel[2] = rng.gen_range(0, 255);
            pixel[3] = rng.gen_range(0, 255);

            img.put_pixel(x_coordinate, y_coordinate, pixel);
        }
    }

    img.save("output.jpg").unwrap();
}
