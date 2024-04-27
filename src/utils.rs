#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

/// Outputs a vector of RGBA bytes as a png image with the given dimensions on the given path.
#[cfg(not(target_arch = "wasm32"))]
pub fn output_image_native(image_data: Vec<u8>, texture_dims: (usize, usize), path: String) {
    let mut png_data = Vec::<u8>::with_capacity(image_data.len());
    let mut encoder = png::Encoder::new(
        std::io::Cursor::new(&mut png_data),
        texture_dims.0 as u32,
        texture_dims.1 as u32,
    );
    encoder.set_color(png::ColorType::Rgba);
    let mut png_writer = encoder.write_header().unwrap();
    png_writer.write_image_data(&image_data[..]).unwrap();
    png_writer.finish().unwrap();
    log::info!("PNG file encoded in memory.");

    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(&png_data[..]).unwrap();
    log::info!("PNG file written to disc as \"{}\".", path);
}