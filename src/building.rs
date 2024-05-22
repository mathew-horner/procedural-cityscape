use std::ops::Range;

use image::{ImageBuffer, Rgb};
use rand::prelude::*;

use crate::common::{put_pixel_safe, Color};
use crate::math::{Dimensions2, Point2};
use crate::window::{Window, WindowType};

pub struct Building {
    pub x: u32,
    pub dimensions: Dimensions2<u32>,
    pub color: Color,
    pub windows: Vec<Window>,
}

pub struct GenerateOpts<'a> {
    pub x: u32,
    pub size_range: Dimensions2<Range<u32>>,
    pub window_margin: u32,
    pub color_opts: &'a [Color],
    pub image_height: u32,
}

pub struct RenderOpts<'a> {
    pub image: &'a mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub building_border_width: u32,
    pub window_border_width: u32,
    pub image_height: u32,
}

impl Building {
    pub fn generate(opts: GenerateOpts<'_>) -> Self {
        let mut rng = rand::thread_rng();
        let building_dimensions = generate_size(&opts.size_range, &mut rng);

        let mut windows = Vec::new();
        let window_type = rand::random::<WindowType>();
        let window_dimensions = window_type.dimensions_for(building_dimensions.width());

        if !window_dimensions.is_zero() {
            let mut y = opts.window_margin;
            while y < opts.image_height {
                let mut x = opts.window_margin;
                for _ in 0..window_type.per_row() {
                    windows.push(Window::new(Point2::new(x, y), window_dimensions.clone()));
                    x += window_dimensions.width() + opts.window_margin;
                }
                y += window_dimensions.height() + opts.window_margin;
            }
        }

        let color = opts.color_opts[rng.gen_range(0..opts.color_opts.len())];

        Self {
            x: opts.x,
            dimensions: building_dimensions,
            color,
            windows,
        }
    }

    pub fn render(&self, opts: RenderOpts<'_>) {
        self.render_rectangle(
            opts.image,
            0,
            0,
            self.dimensions.height(),
            self.dimensions.width(),
            self.color.clone(),
            opts.image_height,
        );
        self.render_rectangle(
            opts.image,
            0,
            0,
            opts.building_border_width,
            self.dimensions.width(),
            (0, 0, 0),
            opts.image_height,
        );
        self.render_rectangle(
            opts.image,
            0,
            0,
            self.dimensions.height(),
            opts.building_border_width,
            (0, 0, 0),
            opts.image_height,
        );
        self.render_rectangle(
            opts.image,
            0,
            self.dimensions.width() - opts.building_border_width,
            self.dimensions.height(),
            opts.building_border_width,
            (0, 0, 0),
            opts.image_height,
        );

        for window in self.windows.iter() {
            self.render_rectangle(
                opts.image,
                window.position().y(),
                window.position().x(),
                window.dimensions().height(),
                window.dimensions().width(),
                (120, 120, 120),
                opts.image_height,
            );
            self.render_rectangle(
                opts.image,
                window.position().y(),
                window.position().x(),
                opts.window_border_width,
                window.dimensions().width(),
                (0, 0, 0),
                opts.image_height,
            );
            self.render_rectangle(
                opts.image,
                window.position().y() + window.dimensions().height() - opts.window_border_width,
                window.position().x(),
                opts.window_border_width,
                window.dimensions().width(),
                (0, 0, 0),
                opts.image_height,
            );
            self.render_rectangle(
                opts.image,
                window.position().y(),
                window.position().x(),
                window.dimensions().height(),
                opts.window_border_width,
                (0, 0, 0),
                opts.image_height,
            );
            self.render_rectangle(
                opts.image,
                window.position().y(),
                window.position().x() + window.dimensions().width() - opts.window_border_width,
                window.dimensions().height(),
                opts.window_border_width,
                (0, 0, 0),
                opts.image_height,
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
        image_height: u32,
    ) {
        for row in start_row..start_row + height {
            for col in start_col..start_col + width {
                let (x, y) = self.to_screen_space(col, row, image_height);
                put_pixel_safe(image, x, y, color.clone());
            }
        }
    }

    fn to_screen_space(&self, x: u32, y: u32, image_height: u32) -> (u32, u32) {
        (self.x + x, image_height - self.dimensions.height() + y)
    }
}

/// Generate a random size based off a given range of sizes.
fn generate_size(size_range: &Dimensions2<Range<u32>>, rng: &mut ThreadRng) -> Dimensions2<u32> {
    let height = rng.gen_range(size_range.height_ref().clone());
    let width = rng.gen_range(size_range.width_ref().clone());
    Dimensions2::new(height, width)
}
