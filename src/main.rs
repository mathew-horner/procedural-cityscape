mod building;
mod common;
mod window;

use building::Building;
use common::{BUILDING_OFFSET_RANGE, IMAGE_HEIGHT, IMAGE_WIDTH};
use image::{ImageBuffer, ImageFormat};
use rand::prelude::*;
use std::cmp::max;

fn main() {
    let mut col = 0;
    let mut buildings = Vec::new();
    let mut rng = rand::thread_rng();

    while col < IMAGE_WIDTH {
        let offset = rng.gen_range(BUILDING_OFFSET_RANGE);
        let position = max(0, col as i32 + offset) as u32;
        if position >= IMAGE_WIDTH { break; }
        let building = Building::generate(position);
        // TODO: Can this cause bad things? I have a sneaky suspicion...
        col += (building.width as i32 + offset) as u32;
        buildings.push(building);
    }

    if let Some(last) = buildings.last_mut() {
        last.width = IMAGE_WIDTH - last.x - 1;
    }

    buildings.shuffle(&mut rng);
    let mut image = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for building in buildings.iter() {
        building.render(&mut image);
    }

    image
        .save_with_format("render.bmp", ImageFormat::Bmp)
        .unwrap();
}