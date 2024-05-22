mod building;
mod common;
mod math;
mod window;

use std::cmp::max;

use image::{ImageBuffer, ImageFormat, Rgb};
use rand::prelude::*;

use crate::building::Building;
use crate::common::{BUILDING_OFFSET_RANGE, IMAGE_HEIGHT, IMAGE_WIDTH};

const STAR_PRESENCE_PROBABILITY: f64 = 0.7;

const STAR_CELL_COUNT_HORIZONTAL: u32 = 30;

const STAR_CELL_COUNT_VERTICAL: u32 =
    (STAR_CELL_COUNT_HORIZONTAL as f64 * (IMAGE_HEIGHT as f64 / IMAGE_WIDTH as f64)) as u32;

const STAR_CELL_WIDTH: u32 = (IMAGE_WIDTH as f64 / STAR_CELL_COUNT_HORIZONTAL as f64) as u32;

const STAR_CELL_HEIGHT: u32 = (IMAGE_HEIGHT as f64 / STAR_CELL_COUNT_VERTICAL as f64) as u32;

const STAR_BIG_PROBABILITY: f64 = 0.3;

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
        let building = Building::generate(position);
        // TODO: Can this cause bad things? I have a sneaky suspicion...
        col += (building.dimensions.width() as i32 + offset) as u32;
        buildings.push(building);
    }

    if let Some(last) = buildings.last_mut() {
        *last.dimensions.width_mut() = IMAGE_WIDTH - last.x - 1;
    }

    let mut image = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for row in 0..STAR_CELL_COUNT_VERTICAL {
        for col in 0..STAR_CELL_COUNT_HORIZONTAL {
            if rng.gen::<f64>() <= STAR_PRESENCE_PROBABILITY {
                let star_size = if rng.gen::<f64>() <= STAR_BIG_PROBABILITY {
                    2
                } else {
                    1
                };
                let x = (col * STAR_CELL_WIDTH) + rng.gen_range(0..STAR_CELL_WIDTH);
                let y = (row * STAR_CELL_HEIGHT) + rng.gen_range(0..STAR_CELL_HEIGHT);
                for i in 0..star_size {
                    for j in 0..star_size {
                        image.put_pixel(x + i, y + j, Rgb([255, 255, 120]));
                    }
                }
            }
        }
    }

    buildings.shuffle(&mut rng);
    for building in buildings.iter() {
        building.render(&mut image);
    }

    image
        .save_with_format("render.bmp", ImageFormat::Bmp)
        .unwrap();
}

