use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    pub fn add(mut self, other: &Vector) -> Vector {
        self.0 += other.0;
        self.1 += other.1;
        self
    }

    pub fn subtract(mut self, other: &Vector) -> Vector {
        self.0 -= other.0;
        self.1 -= other.1;
        self
    }

    pub fn scale(mut self, scalar: f32) -> Vector {
        self.0 *= scalar;
        self.1 *= scalar;
        self
    }

    pub fn multiply(mut self, other: &Vector) -> Vector {
        (self.0, self.1) = (
            self.0 * other.0 - self.1 * other.1,
            self.0 * other.1 + self.1 * other.0,
        );
        self
    }

    pub fn square_magnitude(&self) -> f32 {
        self.0.powf(2.0) + self.1.powf(2.0)
    }

    pub fn magnitude(&self) -> f32 {
        self.square_magnitude().sqrt()
    }

    pub fn square_distance(&self, other: &Vector) -> f32 {
        (self.0 - other.0).powf(2.0) + (self.1 - other.1).powf(2.0)
    }

    pub fn distance(&self, other: &Vector) -> f32 {
        self.square_distance(other).sqrt()
    }

    pub fn close_to(&self, other: &Vector, radius: f32) -> bool {
        self.square_distance(other) <= radius.powf(2.0)
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Vector ({}, {})", self.0, self.1)
    }
}
