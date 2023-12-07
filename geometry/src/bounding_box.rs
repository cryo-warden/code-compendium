use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct BoundingBox {
    left: f32,
    bottom: f32,
    right: f32,
    top: f32,
}

impl BoundingBox {
    pub fn new(left: f32, bottom: f32, right: f32, top: f32) -> BoundingBox {
        BoundingBox {
            left,
            bottom,
            right,
            top,
        }
    }

    pub fn area(&self) -> f32 {
        (self.right - self.left) * (self.top - self.bottom)
    }

    pub fn touches(&self, b: &BoundingBox) -> bool {
        self.right >= b.left && self.left <= b.right && self.top >= b.bottom && self.bottom <= b.top
    }

    pub fn is_valid(&self) -> bool {
        self.left <= self.right && self.bottom <= self.top
    }

    pub fn merge(&self, b: &BoundingBox) -> BoundingBox {
        BoundingBox {
            left: self.left.min(b.left),
            bottom: self.bottom.min(b.bottom),
            right: self.right.max(b.right),
            top: self.top.max(b.top),
        }
    }
}

impl Display for BoundingBox {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "BoundingBox ({}, {}, {}, {})",
            self.left, self.bottom, self.right, self.top
        )
    }
}
