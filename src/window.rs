use super::common::WINDOW_MARGIN;
use rand::{prelude::*, distributions::{Distribution, Standard}};
use std::cmp::max;

pub struct Window {
    pub x: u32,
    pub y: u32,
    pub height: u32,
    pub width: u32,
}

impl Window {
    pub fn new(x: u32, y: u32, height: u32, width: u32) -> Self {
        Window { x, y, height, width }
    }
}

// TODO: Add WindowType::OneByTwo?
pub enum WindowType {
    /// [     ]
    /// [     ]
    TwoByTwo,
    /// [ ] [ ]
    /// [ ] [ ]
    OneByTwo,
    /// [     ]
    TwoByOne,
    /// [ ] [ ]
    OneByOne,

}

impl WindowType {
    pub fn dimensions_for(&self, building_width: u32) -> (u32, u32) {
        let (height, width) = match self {
            Self::TwoByTwo => {
                let size = building_width as i32 - (WINDOW_MARGIN * 2) as i32;
                (size, size)
            },
            Self::OneByTwo => {
                let width = (building_width as i32 - (WINDOW_MARGIN * 3) as i32) / 2;
                (width * 2, width)
            }
            Self::TwoByOne => {
                let width = building_width as i32 - (WINDOW_MARGIN * 2) as i32;
                (width / 2, width)
            }
            Self::OneByOne => {
                let size = (building_width as i32 - (WINDOW_MARGIN * 3) as i32) / 2;
                (size, size)
            }
        };
        (max(0, height) as u32, max(0, width) as u32)
    }

    pub fn per_row(&self) -> u32 {
        match self {
            Self::TwoByTwo | Self::TwoByOne => 1,
            Self::OneByTwo | Self::OneByOne => 2,
        }
    }
}

const WINDOW_TYPE_COUNT: u32 = 4;

impl Distribution<WindowType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> WindowType {
        match rng.gen_range(0..WINDOW_TYPE_COUNT) {
            0 => WindowType::TwoByTwo,
            1 => WindowType::OneByTwo,
            2 => WindowType::TwoByOne,
            _ => WindowType::OneByOne,
        }
    }
}
