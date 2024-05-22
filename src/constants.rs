use std::ops::Range;

use crate::common::Color;

pub const IMAGE_HEIGHT: u32 = 1080;

pub const IMAGE_WIDTH: u32 = 1900;

pub const BUILDING_COLORS: [Color; 4] =
    [(30, 30, 30), (80, 80, 80), (200, 200, 200), (175, 175, 175)];

pub const BUILDING_HEIGHT_RANGE: Range<u32> = 500..900;

pub const BUILDING_WIDTH_RANGE: Range<u32> = 200..275;

pub const BUILDING_OFFSET_RANGE: Range<i32> = -100..50;

pub const BUILDING_BORDER_THICKNESS: u32 = 5;

// TODO: This is a temporary hack. Eventually we will want to determine window margin off of building width.
pub const WINDOW_MARGIN: u32 = 50;

pub const WINDOW_BORDER_THICKNESS: u32 = 5;

pub const STAR_PRESENCE_PROBABILITY: f64 = 0.7;

pub const STAR_CELL_COUNT_HORIZONTAL: u32 = 30;

pub const STAR_CELL_COUNT_VERTICAL: u32 =
    (STAR_CELL_COUNT_HORIZONTAL as f64 * (IMAGE_HEIGHT as f64 / IMAGE_WIDTH as f64)) as u32;

pub const STAR_CELL_WIDTH: u32 = (IMAGE_WIDTH as f64 / STAR_CELL_COUNT_HORIZONTAL as f64) as u32;

pub const STAR_CELL_HEIGHT: u32 = (IMAGE_HEIGHT as f64 / STAR_CELL_COUNT_VERTICAL as f64) as u32;

pub const STAR_BIG_PROBABILITY: f64 = 0.3;
