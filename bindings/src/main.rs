mod image_processing;

fn main(){
    let img = image::open("sample/fuji.jpg").unwrap();

    image_processing::grayscale(img).save("output.png");
}
