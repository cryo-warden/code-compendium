use std::{
    f32::consts::PI,
    fmt::{Display, Formatter, Result},
};

use crate::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Circle {
    pub center: Vector,
    pub radius: f32,
}

impl Circle {
    pub fn area(&self) -> f32 {
        self.radius.powf(2.0) * PI
    }

    pub fn circumference(&self) -> f32 {
        2.0 * self.radius * PI
    }

    pub fn contains(&self, v: Vector) -> bool {
        self.center.close_to(v, self.radius)
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Circle {} ({}, {})",
            self.radius, self.center.0, self.center.1
        )
    }
}
