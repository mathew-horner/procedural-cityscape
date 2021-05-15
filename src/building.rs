use super::{
    common::{BUILDING_COLORS, BUILDING_HEIGHT_RANGE, BUILDING_WIDTH_RANGE, Color, IMAGE_HEIGHT, put_pixel_safe, WINDOW_MARGIN, WINDOW_BORDER_THICKNESS},
    window::{Window, WindowType},
};
use image::{ImageBuffer, Rgb};
use rand::prelude::*;

pub struct Building {
    pub x: u32,
    pub height: u32,
    pub width: u32,
    pub color: Color,
    pub windows: Vec<Window>,
}

impl Building {
    pub fn generate(x: u32) -> Self {
        let mut rng = rand::thread_rng();
        let building_width = rng.gen_range(BUILDING_WIDTH_RANGE.clone());
        let building_height = rng.gen_range(BUILDING_HEIGHT_RANGE.clone());

        let mut windows = Vec::new();
        let window_type = rand::random::<WindowType>();
        let (window_height, window_width) = window_type.dimensions_for(building_width);

        if window_height > 0 && window_width > 0 {
            let mut y = WINDOW_MARGIN;
            while y < IMAGE_HEIGHT {
                let mut x = WINDOW_MARGIN;
                for _ in 0..window_type.per_row() {
                    windows.push(Window::new(x, y, window_height, window_width));
                    x += window_width + WINDOW_MARGIN;
                }
                y += window_height + WINDOW_MARGIN;
            }
        }

        Self {
            x,
            height: building_height,
            width: building_width,
            color: BUILDING_COLORS[rng.gen_range(0..BUILDING_COLORS.len())],
            windows,
        }
    }

    pub fn render(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
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