use std::cmp::max;
use std::ops::Range;

use image::{ImageBuffer, Rgb};
use rand::prelude::*;

use crate::common::{put_pixel_safe, Color};
use crate::math::{Dimensions2, Point2};
use crate::window::{Window, WindowType};

/// Parameters for `skyline::render`.
pub struct RenderOpts<'a> {
    /// The image to render to.
    pub image: &'a mut ImageBuffer<Rgb<u8>, Vec<u8>>,

    /// Random number generator to use.
    pub rng: &'a mut ThreadRng,

    /// The pixel width of the building's drawn borders.
    pub building_border_thickness: u32,

    /// The range of offsets to randomize amongst when placing buildings.
    pub building_offset_range: Range<i32>,

    /// The range of sizes to randomize amongst when sizing buildings.
    pub building_size_range: Dimensions2<Range<u32>>,

    /// The pixel width of the windows' drawn borders.
    pub window_border_thickness: u32,

    /// The distance between windows and the edge of the building.
    pub window_margin: u32,

    /// The colors to choose from when drawing the building.
    pub color_opts: &'a [Color],
}

/// Render a randomized city skyline to the given image.
pub fn render(opts: RenderOpts<'_>) {
    let mut col = 0;
    let mut buildings = Vec::new();

    while col < opts.image.width() {
        let offset = opts.rng.gen_range(opts.building_offset_range.clone());
        let position = max(0, col as i32 + offset) as u32;
        if position >= opts.image.width() {
            break;
        }
        let building = Building::generate(BuildingGenerateOpts {
            x: position,
            size_range: opts.building_size_range.clone(),
            window_margin: opts.window_margin,
            color_opts: opts.color_opts,
            image_height: opts.image.height(),
        });
        // TODO: Can this cause bad things? I have a sneaky suspicion...
        col += (building.dimensions.width() as i32 + offset) as u32;
        buildings.push(building);
    }

    if let Some(last) = buildings.last_mut() {
        *last.dimensions.width_mut() = opts.image.width() - last.x - 1;
    }

    // We shuffle the array first so that there is randomization in terms of which buildings
    // overlap each other.
    buildings.shuffle(opts.rng);

    for building in buildings.iter() {
        building.render(BuildingRenderOpts {
            image: opts.image,
            building_border_thickness: opts.building_border_thickness,
            window_border_thickness: opts.window_border_thickness,
        });
    }
}

struct Building {
    x: u32,
    dimensions: Dimensions2<u32>,
    color: Color,
    windows: Vec<Window>,
}

struct BuildingGenerateOpts<'a> {
    x: u32,
    size_range: Dimensions2<Range<u32>>,
    window_margin: u32,
    color_opts: &'a [Color],
    image_height: u32,
}

struct BuildingRenderOpts<'a> {
    image: &'a mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    building_border_thickness: u32,
    window_border_thickness: u32,
}

impl Building {
    fn generate(opts: BuildingGenerateOpts<'_>) -> Self {
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

    fn render(&self, opts: BuildingRenderOpts<'_>) {
        self.render_rectangle(
            opts.image,
            0,
            0,
            self.dimensions.height(),
            self.dimensions.width(),
            self.color.clone(),
            opts.image.height(),
        );
        self.render_rectangle(
            opts.image,
            0,
            0,
            opts.building_border_thickness,
            self.dimensions.width(),
            (0, 0, 0),
            opts.image.height(),
        );
        self.render_rectangle(
            opts.image,
            0,
            0,
            self.dimensions.height(),
            opts.building_border_thickness,
            (0, 0, 0),
            opts.image.height(),
        );
        self.render_rectangle(
            opts.image,
            0,
            self.dimensions.width() - opts.building_border_thickness,
            self.dimensions.height(),
            opts.building_border_thickness,
            (0, 0, 0),
            opts.image.height(),
        );

        for window in self.windows.iter() {
            self.render_rectangle(
                opts.image,
                window.position().y(),
                window.position().x(),
                window.dimensions().height(),
                window.dimensions().width(),
                (120, 120, 120),
                opts.image.height(),
            );
            self.render_rectangle(
                opts.image,
                window.position().y(),
                window.position().x(),
                opts.window_border_thickness,
                window.dimensions().width(),
                (0, 0, 0),
                opts.image.height(),
            );
            self.render_rectangle(
                opts.image,
                window.position().y() + window.dimensions().height() - opts.window_border_thickness,
                window.position().x(),
                opts.window_border_thickness,
                window.dimensions().width(),
                (0, 0, 0),
                opts.image.height(),
            );
            self.render_rectangle(
                opts.image,
                window.position().y(),
                window.position().x(),
                window.dimensions().height(),
                opts.window_border_thickness,
                (0, 0, 0),
                opts.image.height(),
            );
            self.render_rectangle(
                opts.image,
                window.position().y(),
                window.position().x() + window.dimensions().width() - opts.window_border_thickness,
                window.dimensions().height(),
                opts.window_border_thickness,
                (0, 0, 0),
                opts.image.height(),
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
