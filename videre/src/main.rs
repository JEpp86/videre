use std::process;

use image::ImageReader;
// use vid_src::image_source;

fn main() {
    let _input_img = ImageReader::open("im0.png").unwrap_or_else(|err| {
        eprintln!("Image read failed {err}");
        process::exit(1);
    });
    println!("Image Read successful")
}
