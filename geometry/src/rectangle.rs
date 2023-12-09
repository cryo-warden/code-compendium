use std::fmt::{Display, Formatter, Result};

use crate::BoundingBox;
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

    pub fn contains(&self, v: Vector) -> bool {
        let offset = v - self.center;
        2.0 * offset.0.abs() <= self.width && 2.0 * offset.1.abs() <= self.height
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let half_dimensions = Vector(self.width, self.height) / 2.0;
        let min = self.center - half_dimensions;
        let max = self.center + half_dimensions;
        BoundingBox::new(min.0, min.1, max.0, max.1)
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
