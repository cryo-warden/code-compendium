use std::fmt::{Display, Formatter, Result};

use crate::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rectangle {
    pub center: Vector,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f32 {
        2.0 * (self.width + self.height)
    }

    pub fn contains(&self, v: &Vector) -> bool {
        2.0 * (v.0 - self.center.0).abs() <= self.width
            && 2.0 * (v.1 - self.center.1).abs() <= self.height
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Rectangle {}x{} ({}, {})",
            self.width, self.height, self.center.0, self.center.1
        )
    }
}
