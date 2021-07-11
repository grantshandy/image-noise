use std::io::prelude::*;
use std::fs::File;

fn main() {
    let image = image_noise::noise_image(1000, 10);

    let mut file = File::create("image.png").unwrap();

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut bytes, image::ImageOutputFormat::Png)
        .expect("Can write to png");

    file.write_all(&bytes).unwrap();
}