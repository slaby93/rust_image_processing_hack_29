use std::fs;

mod image_processing;

fn main() {
    let img = image::open("/Users/apau/Desktop/Klay_Patrick.jpg").unwrap();
    let watermark = image::open("sample/watermark.png").unwrap();

    fs::create_dir_all("outputs").unwrap();
    image_processing::grayscale(&img)
        .save("outputs/grayscale.jpg")
        .unwrap();
    image_processing::add_watermark(&img, &watermark, 0.5)
        .save("outputs/watermark.jpg")
        .unwrap();
    image_processing::detect_edges(&img)
        .save("outputs/detect_edges.jpg")
        .unwrap();
    image_processing::add_noise(&img)
        .save("outputs/add_noise.jpg")
        .unwrap();
    image_processing::invert(&img)
        .save("outputs/invert.jpg")
        .unwrap();
    image_processing::pixellate(&img)
        .save("outputs/pixellate.jpg")
        .unwrap();
    image_processing::enhance_edges(&img)
        .save("outputs/enhance_edges.jpg")
        .unwrap();
    image_processing::detail(&img)
        .save("outputs/detail.jpg")
        .unwrap();
    image_processing::blur(&img)
        .save("outputs/blur.jpg")
        .unwrap();
}
