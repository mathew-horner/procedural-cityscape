use image::{ImageBuffer, Rgb};

pub type Color = (u8, u8, u8);

// TODO: Make this not fail silently.
pub fn put_pixel_safe(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32, color: Color) {
    if x >= crate::constants::IMAGE_WIDTH || y >= crate::constants::IMAGE_HEIGHT {
        return;
    }
    image.put_pixel(x, y, Rgb([color.0, color.1, color.2]));
}

