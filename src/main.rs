mod common;
mod constants;
mod math;
mod nightsky;
mod skyline;
mod window;

use image::{ImageBuffer, ImageFormat, Rgb};
use math::{Dimensions2, Vector2};

use crate::constants::*;

fn main() {
    let mut image = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut rng = rand::thread_rng();

    nightsky::render(nightsky::RenderOpts {
        image: &mut image,
        rng: &mut rng,
        cell_count: Vector2::new(STAR_CELL_COUNT_HORIZONTAL, STAR_CELL_COUNT_VERTICAL),
        cell_size: Dimensions2::new(STAR_CELL_HEIGHT, STAR_CELL_WIDTH),
        star_presence_prob: STAR_PRESENCE_PROBABILITY,
        star_enlargement_prob: STAR_BIG_PROBABILITY,
        star_color: Rgb([255, 255, 120]),
    });

    skyline::render(skyline::RenderOpts {
        image: &mut image,
        rng: &mut rng,
        building_border_thickness: BUILDING_BORDER_THICKNESS,
        building_offset_range: BUILDING_OFFSET_RANGE,
        building_size_range: Dimensions2::new(BUILDING_HEIGHT_RANGE, BUILDING_WIDTH_RANGE),
        window_border_thickness: WINDOW_BORDER_THICKNESS,
        window_margin: WINDOW_MARGIN,
        color_opts: &BUILDING_COLORS,
    });

    image
        .save_with_format("render.bmp", ImageFormat::Bmp)
        .unwrap();
}
