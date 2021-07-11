//! # image-noise
//! Creates A simple perlin noise texture in Rust.
//!
//! ```
//! image-noise = "0.1.0"
//! ```
//1
//! ```rust
//! let image = image_noise::noise_image(1000, 10);
//!
//! let mut file = File::create("image.png").unwrap();
//!
//! let mut bytes: Vec<u8> = Vec::new();
//! image
//!     .write_to(&mut bytes, image::ImageOutputFormat::Png)
//!     .expect("Can write to png");
//! 
//! file.write_all(&bytes).unwrap();
//! ```

use image::{DynamicImage, ImageBuffer, Rgb};
use txture::PerlinNoise;

/// Create a noise image with the size in width and height, and the resolution of the noise.
pub fn noise_image(s: u32, gradient_point_number: u32) -> DynamicImage {
    // Create perlin noise
    let perlin_noise = PerlinNoise::new(s, gradient_point_number, true).unwrap();

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(s, s);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let gray: u8 = perlin_noise.get_pixel_value(x, y);

        *pixel = Rgb([gray, gray, gray]);
    }

    let image = DynamicImage::ImageRgb8(imgbuf);

    return image;
}
