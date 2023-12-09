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

impl core::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        self.add(&other)
    }
}

impl core::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        self.subtract(&other)
    }
}

impl core::ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Vector {
        self.scale(scalar)
    }
}

impl core::ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        vector.scale(self)
    }
}

impl core::ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, scalar: f32) -> Vector {
        self.scale(1.0 / scalar)
    }
}

impl core::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        self.scale(-1.0)
    }
}

impl core::ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        self.multiply(&other)
    }
}

impl core::ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = self.add(&other);
    }
}

impl core::ops::SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, other: Vector) {
        *self = self.subtract(&other);
    }
}

impl core::ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, scalar: f32) {
        *self = self.scale(scalar);
    }
}

impl core::ops::DivAssign<f32> for Vector {
    fn div_assign(&mut self, scalar: f32) {
        *self = self.scale(1.0 / scalar);
    }
}

impl core::ops::MulAssign<Vector> for Vector {
    fn mul_assign(&mut self, other: Vector) {
        *self = self.multiply(&other);
    }
}
