use image::{ImageBuffer, Rgb};
use rand::prelude::*;

use crate::common::{
    put_pixel_safe, Color, BUILDING_BORDER_THICKNESS, BUILDING_COLORS, BUILDING_HEIGHT_RANGE,
    BUILDING_WIDTH_RANGE, IMAGE_HEIGHT, WINDOW_BORDER_THICKNESS, WINDOW_MARGIN,
};
use crate::math::{Dimensions2, Point2};
use crate::window::{Window, WindowType};

pub struct Building {
    pub x: u32,
    pub dimensions: Dimensions2<u32>,
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
        let window_dimensions = window_type.dimensions_for(building_width);

        if !window_dimensions.is_zero() {
            let mut y = WINDOW_MARGIN;
            while y < IMAGE_HEIGHT {
                let mut x = WINDOW_MARGIN;
                for _ in 0..window_type.per_row() {
                    windows.push(Window::new(Point2::new(x, y), window_dimensions.clone()));
                    x += window_dimensions.width() + WINDOW_MARGIN;
                }
                y += window_dimensions.height() + WINDOW_MARGIN;
            }
        }

        Self {
            x,
            dimensions: Dimensions2::new(building_height, building_width),
            color: BUILDING_COLORS[rng.gen_range(0..BUILDING_COLORS.len())],
            windows,
        }
    }

    pub fn render(&self, image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        self.render_rectangle(
            image,
            0,
            0,
            self.dimensions.height(),
            self.dimensions.width(),
            self.color.clone(),
        );
        self.render_rectangle(
            image,
            0,
            0,
            BUILDING_BORDER_THICKNESS,
            self.dimensions.width(),
            (0, 0, 0),
        );
        self.render_rectangle(
            image,
            0,
            0,
            self.dimensions.height(),
            BUILDING_BORDER_THICKNESS,
            (0, 0, 0),
        );
        self.render_rectangle(
            image,
            0,
            self.dimensions.width() - BUILDING_BORDER_THICKNESS,
            self.dimensions.height(),
            BUILDING_BORDER_THICKNESS,
            (0, 0, 0),
        );

        for window in self.windows.iter() {
            self.render_rectangle(
                image,
                window.position().y(),
                window.position().x(),
                window.dimensions().height(),
                window.dimensions().width(),
                (120, 120, 120),
            );
            self.render_rectangle(
                image,
                window.position().y(),
                window.position().x(),
                WINDOW_BORDER_THICKNESS,
                window.dimensions().width(),
                (0, 0, 0),
            );
            self.render_rectangle(
                image,
                window.position().y() + window.dimensions().height() - WINDOW_BORDER_THICKNESS,
                window.position().x(),
                WINDOW_BORDER_THICKNESS,
                window.dimensions().width(),
                (0, 0, 0),
            );
            self.render_rectangle(
                image,
                window.position().y(),
                window.position().x(),
                window.dimensions().height(),
                WINDOW_BORDER_THICKNESS,
                (0, 0, 0),
            );
            self.render_rectangle(
                image,
                window.position().y(),
                window.position().x() + window.dimensions().width() - WINDOW_BORDER_THICKNESS,
                window.dimensions().height(),
                WINDOW_BORDER_THICKNESS,
                (0, 0, 0),
            );
        }
    }

    fn render_rectangle(
        &self,
        image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        start_row: u32,
        start_col: u32,
        height: u32,
        width: u32,
        color: Color,
    ) {
        for row in start_row..start_row + height {
            for col in start_col..start_col + width {
                let (x, y) = self.to_screen_space(col, row);
                put_pixel_safe(image, x, y, color.clone());
            }
        }
    }

    fn to_screen_space(&self, x: u32, y: u32) -> (u32, u32) {
        (self.x + x, IMAGE_HEIGHT - self.dimensions.height() + y)
    }
}

