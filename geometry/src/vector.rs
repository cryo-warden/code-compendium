use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    /// Perform the slow, ordinary version of vector normalization.
    pub fn normalize(self) -> Self {
        // I'm aware that fast_inverse_square_root exists, but I don't require dark magic at this time.
        let magnitude = self.magnitude();
        Vector(self.0, self.1) / magnitude
    }

    pub fn square_magnitude(self) -> f32 {
        self.0.powf(2.0) + self.1.powf(2.0)
    }

    pub fn magnitude(self) -> f32 {
        self.square_magnitude().sqrt()
    }

    pub fn square_distance(self, other: Self) -> f32 {
        (self - other).square_magnitude()
    }

    pub fn distance(self, other: Self) -> f32 {
        self.square_distance(other).sqrt()
    }

    pub fn close_to(self, other: Self, radius: f32) -> bool {
        self.square_distance(other) <= radius.powf(2.0)
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Vector ({}, {})", self.0, self.1)
    }
}

impl core::ops::Add<Self> for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector(self.0 + other.0, self.1 + other.1)
    }
}

impl core::ops::Sub<Self> for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector(self.0 - other.0, self.1 - other.1)
    }
}

impl core::ops::Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vector(self.0 * scalar, self.1 * scalar)
    }
}

impl core::ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector(vector.0 * self, vector.1 * self)
    }
}

impl core::ops::Div<f32> for Vector {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        self * (1.0 / scalar)
    }
}

impl core::ops::Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        self * -1.0
    }
}

impl core::ops::Mul<Self> for Vector {
    type Output = Self;

    /// Multiply two vectors together as though they're complex numbers.
    /// This is useful for 2D rotation.
    fn mul(self, other: Self) -> Self {
        Vector(
            self.0 * other.0 - self.1 * other.1,
            self.0 * other.1 + self.1 * other.0,
        )
    }
}

impl core::ops::AddAssign<Self> for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl core::ops::SubAssign<Self> for Vector {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl core::ops::MulAssign<f32> for Vector {
    fn mul_assign(&mut self, scalar: f32) {
        *self = *self * scalar;
    }
}

impl core::ops::DivAssign<f32> for Vector {
    fn div_assign(&mut self, scalar: f32) {
        *self = *self / scalar;
    }
}

impl core::ops::MulAssign<Self> for Vector {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
