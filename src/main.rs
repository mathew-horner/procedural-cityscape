mod building;
mod common;
mod constants;
mod math;
mod nightsky;
mod window;

use std::cmp::max;

use image::{ImageBuffer, ImageFormat, Rgb};
use math::{Dimensions2, Vector2};
use rand::prelude::*;

use crate::building::Building;
use crate::constants::*;

fn main() {
    let mut col = 0;
    let mut buildings = Vec::new();
    let mut rng = rand::thread_rng();

    while col < IMAGE_WIDTH {
        let offset = rng.gen_range(BUILDING_OFFSET_RANGE);
        let position = max(0, col as i32 + offset) as u32;
        if position >= IMAGE_WIDTH {
            break;
        }
        let building = Building::generate(building::GenerateOpts {
            x: position,
            size_range: Dimensions2::new(BUILDING_HEIGHT_RANGE, BUILDING_WIDTH_RANGE),
            window_margin: WINDOW_MARGIN,
            color_opts: &BUILDING_COLORS,
            image_height: IMAGE_HEIGHT,
        });
        // TODO: Can this cause bad things? I have a sneaky suspicion...
        col += (building.dimensions.width() as i32 + offset) as u32;
        buildings.push(building);
    }

    if let Some(last) = buildings.last_mut() {
        *last.dimensions.width_mut() = IMAGE_WIDTH - last.x - 1;
    }

    let mut image = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    nightsky::render(nightsky::RenderOpts {
        image: &mut image,
        rng: &mut rng,
        cell_count: Vector2::new(STAR_CELL_COUNT_HORIZONTAL, STAR_CELL_COUNT_VERTICAL),
        cell_size: Dimensions2::new(STAR_CELL_HEIGHT, STAR_CELL_WIDTH),
        star_presence_prob: STAR_PRESENCE_PROBABILITY,
        star_enlargement_prob: STAR_BIG_PROBABILITY,
        star_color: Rgb([255, 255, 120]),
    });

    // We shuffle the array first so that there is randomization in terms of which buildings
    // overlap each other.
    buildings.shuffle(&mut rng);

    for building in buildings.iter() {
        building.render(building::RenderOpts {
            image: &mut image,
            building_border_width: BUILDING_BORDER_THICKNESS,
            window_border_width: WINDOW_BORDER_THICKNESS,
            image_height: IMAGE_HEIGHT,
        });
    }

    image
        .save_with_format("render.bmp", ImageFormat::Bmp)
        .unwrap();
}
