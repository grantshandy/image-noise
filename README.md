# image-noise
Creates a simple perlin noise texture in Rust.

```
image-noise = "0.1.0"
```

Really, this is a simple wrapper over [txture](https://docs.rs/txture/), making it easier to be used with the [image](https://docs.rs/image/) crate.

```rust
let image = image_noise::noise_image(1000, 10);

let mut file = File::create("image.png").unwrap();

let mut bytes: Vec<u8> = Vec::new();
image
    .write_to(&mut bytes, image::ImageOutputFormat::Png)
    .expect("Can write to png");

file.write_all(&bytes).unwrap();
```

![example image](https://github.com/grantshandy/image-noise/blob/main/image.png?raw=true)