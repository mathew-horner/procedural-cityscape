use image::{ImageBuffer, ImageFormat, Rgb};
use rand::{prelude::*, distributions::{Distribution, Standard}};
use std::cmp::max;
use std::ops::Range;

type Color = (u8, u8, u8);

const IMAGE_HEIGHT: u32 = 1080;
const IMAGE_WIDTH: u32 = 1900;
const COLORS: [Color; 8] = [
    (30, 30, 30),
    (255, 255, 255),
    (255, 0, 0),
    (0, 255, 0),
    (0, 0, 255),
    (255, 255, 0),
    (0, 255, 255),
    (255, 0, 255),
];
const BUILDING_HEIGHT_RANGE: Range<u32> = 500..900;
const BUILDING_WIDTH_RANGE: Range<u32> = 150..300;
const BUILDING_OFFSET_RANGE: Range<i32> = -100..50;
// TODO: This is a temporary hack. Eventually we will want to determine window margin off of building width.
const WINDOW_MARGIN: u32 = 50;
const WINDOW_BORDER_THICKNESS: u32 = 5;

struct Building {
    position: u32,
    height: u32,
    width: u32,
    color: Color,
    window_type: WindowType,
}

impl Building {
    fn generate(position: u32) -> Self {
        let mut rng = rand::thread_rng();
        let width = rng.gen_range(BUILDING_WIDTH_RANGE.clone());
        Self {
            position,
            height: rng.gen_range(BUILDING_HEIGHT_RANGE.clone()),
            width,
            color: COLORS[rng.gen_range(0..COLORS.len())],
            window_type: rand::random(),
        }
    }

    fn render(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        for row in 0..self.height {
            for col in 0..self.width {
                image.put_pixel(
                    self.position + col,
                    IMAGE_HEIGHT - row - 1,
                    Rgb([self.color.0, self.color.1, self.color.2])
                );
            }
        }

        // TODO: Randomize window size?
        // TODO: Dynamically determine window margin based on building dimensions.
        let window_width = self.width as i32 - (WINDOW_MARGIN * 2) as i32;
        if  window_width <= 0 { return; }
        let mut window_width = window_width as u32;
        let mut window_height = window_width;

        match self.window_type {
            WindowType::TwoByOne => { window_height /= 2; },
            WindowType::OneByTwo => { window_width /= 2; },
            _ => (),
        };

        let mut row = IMAGE_HEIGHT - self.height + WINDOW_MARGIN;

        // TODO: Doesn't work with WindowType::OneByTwo
        while row < IMAGE_HEIGHT {
            let start_row = row;
            for _ in 0..window_height {
                if row >= IMAGE_HEIGHT { return; }
                for col in 0..window_width {
                    image.put_pixel(self.position + col as u32 + WINDOW_MARGIN, row, Rgb([120, 120, 120]));
                }
                row += 1;
            }
            self.render_window_borders(image, window_width, window_height, 0, start_row);
            row += WINDOW_MARGIN;
        }
    }

    fn render_window_borders(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, window_width: u32, window_height: u32, col: u32, row: u32) {
        // TODO: This can probably be optimized to not overwrite pixels.
        self.render_window_border_row(image, window_width, row);
        self.render_window_border_row(image, window_width, row + window_height - WINDOW_BORDER_THICKNESS);
        self.render_window_border_col(image, window_height, row, col);
        self.render_window_border_col(image, window_height, row, col + window_width - WINDOW_BORDER_THICKNESS);
    }

    fn render_window_border_row(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, window_width: u32, start_row: u32) {
        let mut row = start_row;
        for _ in 0..WINDOW_BORDER_THICKNESS {
            if row >= IMAGE_HEIGHT { return; }
            for col in 0..window_width {
                image.put_pixel(self.position + col as u32 + WINDOW_MARGIN, row, Rgb([0, 0, 0]));
            }
            row += 1;
        }
    }

    fn render_window_border_col(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, window_height: u32, start_row: u32, start_col: u32) {
        let mut col = start_col;
        for _ in 0..WINDOW_BORDER_THICKNESS {
            for row in 0..window_height {
                if row >= IMAGE_HEIGHT { return; }
                image.put_pixel(self.position + col as u32 + WINDOW_MARGIN, start_row + row, Rgb([0, 0, 0]));
            }
            col += 1;
        }
    }
}

enum WindowType {
    /// [     ]
    /// [     ]
    TwoByTwo,
    /// [     ]
    TwoByOne,
    /// [ ] [ ]
    OneByTwo,
}

const WINDOW_TYPE_COUNT: u32 = 4;

impl Distribution<WindowType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WindowType {
        match rng.gen_range(0..WINDOW_TYPE_COUNT) {
            0 => WindowType::TwoByTwo,
            1 => WindowType::TwoByOne,
            _ => WindowType::OneByTwo,
        }
    }
}

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
        last.width = IMAGE_WIDTH - last.position - 1;
    }

    let mut image = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for building in buildings.iter() {
        building.render(&mut image);
    }

    image
        .save_with_format("render.bmp", ImageFormat::Bmp)
        .unwrap();
}