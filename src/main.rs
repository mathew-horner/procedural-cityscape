use image::{ImageBuffer, ImageFormat, Rgb};
use rand::{prelude::*, distributions::{Distribution, Standard}};
use std::cmp::max;
use std::ops::Range;

type Color = (u8, u8, u8);

const IMAGE_HEIGHT: u32 = 1080;
const IMAGE_WIDTH: u32 = 1900;
const BUILDING_COLORS: [Color; 8] = [
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
    x: u32,
    height: u32,
    width: u32,
    color: Color,
    windows: Vec<Window>,
}

struct Window {
    x: u32,
    y: u32,
    height: u32,
    width: u32,
}

impl Building {
    fn generate(x: u32) -> Self {
        let mut rng = rand::thread_rng();
        let building_width = rng.gen_range(BUILDING_WIDTH_RANGE.clone());
        let building_height = rng.gen_range(BUILDING_HEIGHT_RANGE.clone());

        let mut windows = Vec::new();

        // TODO: Refactor this.
        match rand::random::<WindowType>() {
            // [     ]
            // [     ]
            WindowType::TwoByTwo => {
                let window_size = building_width as i32 - (WINDOW_MARGIN * 2) as i32;
                if window_size > 0 {
                    let window_size = window_size as u32;
                    let mut y = WINDOW_MARGIN;
                    while y < IMAGE_HEIGHT {
                        windows.push(Window::new(WINDOW_MARGIN, y, window_size, window_size));
                        y += window_size + WINDOW_MARGIN;
                    }
                }
            },
            // [     ]
            WindowType::TwoByOne => {
                let window_width = building_width as i32 - (WINDOW_MARGIN * 2) as i32;
                if window_width > 0 {
                    let window_width = window_width as u32;
                    let window_height = window_width / 2;
                    let mut y = WINDOW_MARGIN;
                    while y < IMAGE_HEIGHT {
                        windows.push(Window::new(WINDOW_MARGIN, y, window_height, window_width));
                        y += window_height + WINDOW_MARGIN;
                    }
                }
            },
            // [ ] [ ]
            WindowType::OneByOne => {
                let window_size = (building_width as i32 - (WINDOW_MARGIN * 3) as i32) / 2;
                if window_size > 0 {
                    let window_size = window_size as u32;
                    let mut y = WINDOW_MARGIN;
                    while y < IMAGE_HEIGHT {
                        windows.push(Window::new(WINDOW_MARGIN, y, window_size, window_size));
                        windows.push(Window::new(WINDOW_MARGIN * 2 + window_size, y, window_size, window_size));
                        y += window_size + WINDOW_MARGIN;
                    }
                }
            },
        };

        Self {
            x,
            height: building_height,
            width: building_width,
            color: BUILDING_COLORS[rng.gen_range(0..BUILDING_COLORS.len())],
            windows,
        }
    }

    fn render(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        self.render_rectangle(image, 0, 0, self.height, self.width, self.color.clone());
        for window in self.windows.iter() {
            self.render_rectangle(image, window.y, window.x, window.height, window.width, (120, 120, 120));
            self.render_rectangle(image, window.y, window.x, WINDOW_BORDER_THICKNESS, window.width, (0, 0, 0));
            self.render_rectangle(image, window.y + window.height - WINDOW_BORDER_THICKNESS, window.x, WINDOW_BORDER_THICKNESS, window.width, (0, 0, 0));
            self.render_rectangle(image, window.y, window.x, window.height, WINDOW_BORDER_THICKNESS, (0, 0, 0));
            self.render_rectangle(image, window.y, window.x + window.width - WINDOW_BORDER_THICKNESS, window.height, WINDOW_BORDER_THICKNESS, (0, 0, 0));
        }
    }

    // TODO: Change parameter order?
    fn render_rectangle(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, start_row: u32, start_col: u32, height: u32, width: u32, color: Color) {
        for row in start_row..start_row + height {
            for col in start_col..start_col + width {
                let (x, y) = self.to_screen_space(col, row);
                put_pixel_safe(image, x, y, color.clone());
            }
        }
    }

    fn to_screen_space(&self, x: u32, y: u32) -> (u32, u32) {
        (self.x + x, IMAGE_HEIGHT - self.height + y)
    }
}

impl Window {
    pub fn new(x: u32, y: u32, height: u32, width: u32) -> Self {
        Window { x, y, height, width }
    }
}

// TODO: Add WindowType::OneByTwo?
enum WindowType {
    /// [     ]
    /// [     ]
    TwoByTwo,
    /// [     ]
    TwoByOne,
    /// [ ] [ ]
    OneByOne,
}

const WINDOW_TYPE_COUNT: u32 = 4;

impl Distribution<WindowType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WindowType {
        match rng.gen_range(0..WINDOW_TYPE_COUNT) {
            0 => WindowType::TwoByTwo,
            1 => WindowType::TwoByOne,
            _ => WindowType::OneByOne,
        }
    }
}

// TODO: Make this not fail silently.
fn put_pixel_safe(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32, color: Color) {
    if x >= IMAGE_WIDTH || y >= IMAGE_HEIGHT { return; }
    image.put_pixel(x, y, Rgb([color.0, color.1, color.2]));
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
        last.width = IMAGE_WIDTH - last.x - 1;
    }

    let mut image = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for building in buildings.iter() {
        building.render(&mut image);
    }

    image
        .save_with_format("render.bmp", ImageFormat::Bmp)
        .unwrap();
}