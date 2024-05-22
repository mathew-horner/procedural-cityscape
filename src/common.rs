use std::ops::Range;

use image::{ImageBuffer, Rgb};

pub type Color = (u8, u8, u8);

pub const IMAGE_HEIGHT: u32 = 1080;

pub const IMAGE_WIDTH: u32 = 1900;

pub const BUILDING_COLORS: [Color; 4] =
    [(30, 30, 30), (80, 80, 80), (200, 200, 200), (175, 175, 175)];

pub const BUILDING_HEIGHT_RANGE: Range<u32> = 500..900;

pub const BUILDING_WIDTH_RANGE: Range<u32> = 200..275;

pub const BUILDING_OFFSET_RANGE: Range<i32> = -100..50;

pub const BUILDING_BORDER_THICKNESS: u32 = 5;

// TODO: This is a temporary hack. Eventually we will want to determine window margin off of building width.
pub const WINDOW_MARGIN: u32 = 50;

pub const WINDOW_BORDER_THICKNESS: u32 = 5;

// TODO: Make this not fail silently.
pub fn put_pixel_safe(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32, color: Color) {
    if x >= IMAGE_WIDTH || y >= IMAGE_HEIGHT {
        return;
    }
    image.put_pixel(x, y, Rgb([color.0, color.1, color.2]));
}

