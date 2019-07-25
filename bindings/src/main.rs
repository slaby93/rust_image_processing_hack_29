extern crate image;

use std::path::Path;

mod image_processing;

fn main() {
    let path = "./in.jpg";
//    let flipped_h = image_processing::flip_horizontally(&path);
//    flipped_h.save(&Path::new("./flipped_horizontally.jpg")).ok().expect("Saving horizontal failed");
//
//    let flipped_v = image_processing::flip_vertically(&path);
//    flipped_v.save(&Path::new("./flipped_vertically.jpg")).ok().expect("Saving vertical failed");
//
//    let inverted = image_processing::invert(&path);
//    inverted.save(&Path::new("./inverted.jpg")).ok().expect("Saving inverted failed");

    let grayscaled = image_processing::grayscale(&path);
    grayscaled.save(&Path::new("./grayscaled.jpg")).ok().expect("Saving grayscaled failed");

}
