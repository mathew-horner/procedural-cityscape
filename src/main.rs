use image::{ImageBuffer, ImageFormat, Rgb};
use rand::{prelude::*, distributions::{Distribution, Standard}};
use std::ops::Range;

type Color = (u8, u8, u8);

const HEIGHT: u32 = 1080;
const WIDTH: u32 = 1900;
const COLORS: [Color; 4] = [
    (255, 255, 255),
    (255, 0, 0),
    (0, 255, 0),
    (0, 0, 255),
];
const HEIGHT_RANGE: Range<u32> = 500..800;
const WIDTH_RANGE: Range<u32> = 200..400;

// TODO: This is a temporary hack. Eventually we will want to determine window margin off of building width.
const WINDOW_MARGIN: u32 = 50;

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
        let width = rng.gen_range(WIDTH_RANGE.clone());
        Self {
            position,
            height: rng.gen_range(HEIGHT_RANGE.clone()),
            width,
            color: COLORS[rng.gen_range(0..COLORS.len())],
            window_type: WindowType::OneByOne, //rand::random(),
        }
    }

    fn render(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        for row in 0..self.height {
            for col in 0..self.width {
                image.put_pixel(
                    self.position + col,
                    HEIGHT - row - 1,
                    Rgb([self.color.0, self.color.1, self.color.2])
                );
            }
        }

        match self.window_type {
            WindowType::OneByOne => {
                // TODO: Randomize window size?
                let window_size = self.width as i32 - (WINDOW_MARGIN * 2) as i32;
                if window_size > 0 {
                    let mut row = 0;
                    while row < self.height - window_size as u32 - WINDOW_MARGIN {
                        for _ in 0..window_size {
                            let y = HEIGHT - self.height + WINDOW_MARGIN + row;
                            if y >= HEIGHT { return; }
                            for j in 0..window_size {
                                image.put_pixel(self.position + j as u32 + WINDOW_MARGIN, y, Rgb([0, 0, 0]));
                            }
                            row += 1;
                        }
                        row += WINDOW_MARGIN;
                    }
                }
            }
            _ => (),
        }
    }
}

enum WindowType {
    TwoByTwo,
    TwoByOne,
    OneByTwo,
    OneByOne,
}

const WINDOW_TYPE_COUNT: u32 = 4;

impl Distribution<WindowType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WindowType {
        match rng.gen_range(0..WINDOW_TYPE_COUNT) {
            0 => WindowType::TwoByTwo,
            1 => WindowType::TwoByOne,
            2 => WindowType::OneByTwo,
            _ => WindowType::OneByOne,
        }
    }
}

fn main() {
    let mut col = 0;
    let mut buildings = Vec::new();

    while col < WIDTH {
        let building = Building::generate(col);
        col += &building.width;
        buildings.push(building);
    }

    if let Some(last) = buildings.last_mut() {
        last.width = WIDTH - last.position - 1;
    }

    let mut image = ImageBuffer::new(WIDTH, HEIGHT);
    for building in buildings.iter() {
        building.render(&mut image);
    }

    image
        .save_with_format("render.bmp", ImageFormat::Bmp)
        .unwrap();
}