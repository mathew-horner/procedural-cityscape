use std::cmp::max;

use rand::distributions::{Distribution, Standard};
use rand::prelude::*;

use crate::constants::*;
use crate::math::{Dimensions2, Point2};

pub struct Window {
    position: Point2<u32>,
    dimensions: Dimensions2<u32>,
}

impl Window {
    pub fn new(position: Point2<u32>, dimensions: Dimensions2<u32>) -> Self {
        Self {
            position,
            dimensions,
        }
    }

    pub fn position(&self) -> &Point2<u32> {
        &self.position
    }

    pub fn dimensions(&self) -> &Dimensions2<u32> {
        &self.dimensions
    }
}

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
    /// For a building with the given size, return the size that this window type should be.
    pub fn dimensions_for(&self, building_width: u32) -> Dimensions2<u32> {
        let (height, width) = match self {
            Self::TwoByTwo => {
                let size = building_width as i32 - (WINDOW_MARGIN * 2) as i32;
                (size, size)
            }
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
        Dimensions2::new(max(0, height) as u32, max(0, width) as u32)
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
