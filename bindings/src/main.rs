use std::fs;

mod image_processing;

fn main(){
    let img = image::open("sample/fuji.jpg").unwrap();
    let watermark = image::open("sample/watermark.png").unwrap();

    fs::create_dir_all("outputs");
    image_processing::grayscale(&img).save("outputs/grayscale.png");
    image_processing::add_watermark(&img, &watermark, 0.5).save("outputs/fuji_and_watermark.png");
    image_processing::detect_edges(&img).save("outputs/detect_edges.png");
}
