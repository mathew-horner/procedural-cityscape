#![allow(unused)]

use std::ops::{Deref, DerefMut};

use num::Zero;

/// A base type for 2-dimensional vector types.
///
/// This type is not meant to be used directly, please use [`Vector2`], [`Point2`], or
/// [`Dimensions2`].
#[derive(Clone)]
pub struct Vector2Base<T> {
    x: T,
    y: T,
}

impl<T: Zero> Vector2Base<T> {
    /// Returns whether either component is zero.
    pub fn is_zero(&self) -> bool {
        self.x.is_zero() || self.y.is_zero()
    }
}

/// A point in 2-dimensional space.
pub type Point2<T> = Vector2<T>;

/// A 2-dimensionsal vector type.
#[derive(Clone)]
pub struct Vector2<T>(Vector2Base<T>);

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(Vector2Base { x, y })
    }

    /// Get an immutable reference to this vector's `x` component.
    pub fn x_ref(&self) -> &T {
        &self.x
    }

    /// Get an immutable reference to this vector's `y` component.
    pub fn y_ref(&self) -> &T {
        &self.y
    }

    /// Get a mutable reference to this vector's `x` component.
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    /// Get a mutable reference to this vector's `y` component.
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }
}

impl<T: Copy> Vector2<T> {
    /// Get this vector's `x` component.
    pub fn x(&self) -> T {
        self.x
    }

    /// Get this vector's `y` component.
    pub fn y(&self) -> T {
        self.y
    }
}

impl<T> Deref for Vector2<T> {
    type Target = Vector2Base<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector2<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A size in 2-dimensional space.
#[derive(Clone)]
pub struct Dimensions2<T>(Vector2Base<T>);

impl<T> Dimensions2<T> {
    pub fn new(height: T, width: T) -> Self {
        Self(Vector2Base {
            x: height,
            y: width,
        })
    }

    /// Get an immutable reference to the `height` dimension.
    pub fn height_ref(&self) -> &T {
        &self.x
    }

    /// Get an immutable reference to the `width` dimension.
    pub fn width_ref(&self) -> &T {
        &self.y
    }

    /// Get a mutable reference to the `height` dimension.
    pub fn height_mut(&mut self) -> &mut T {
        &mut self.x
    }

    /// Get a mutable reference to the `width` dimension.
    pub fn width_mut(&mut self) -> &mut T {
        &mut self.y
    }
}

impl<T: Copy> Dimensions2<T> {
    /// Get the `height` dimension.
    pub fn height(&self) -> T {
        self.x
    }

    /// Get the `width` dimension.
    pub fn width(&self) -> T {
        self.y
    }
}

impl<T> Deref for Dimensions2<T> {
    type Target = Vector2Base<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Dimensions2<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
