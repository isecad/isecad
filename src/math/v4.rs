#![allow(dead_code)]

use crate::*;

/// A 4D vector.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(align(16))]
pub struct V4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<[f32; 4]> for V4 {
    fn from([x, y, z, w]: [f32; 4]) -> Self {
        Self { x, y, z, w }
    }
}

impl Into<[f32; 4]> for V4 {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

/// Vector negation.
impl std::ops::Neg for V4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

/// Vector addition.
impl std::ops::Add for V4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

/// Vector addition with assignment.
impl std::ops::AddAssign for V4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Vector subtraction.
impl std::ops::Sub for V4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

/// Vector subtraction with assignment.
impl std::ops::SubAssign for V4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Vector multiplication by scalar.
impl std::ops::Mul<f32> for V4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

/// Vector multiplication by scalar with assignment.
impl std::ops::MulAssign<f32> for V4 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = *self * scalar;
    }
}

/// Vector multiplication by M4.
impl std::ops::Mul<M4> for V4 {
    type Output = Self;

    fn mul(self, rhs: M4) -> Self::Output {
        let Self { x, y, z, w } = self;

        Self {
            x: rhs.a.x * x + rhs.b.x * y + rhs.c.x * z + rhs.d.x * w,
            y: rhs.a.y * x + rhs.b.y * y + rhs.c.y * z + rhs.d.y * w,
            z: rhs.a.z * x + rhs.b.z * y + rhs.c.z * z + rhs.d.z * w,
            w: rhs.a.w * x + rhs.b.w * y + rhs.c.w * z + rhs.d.w * w,
        }
    }
}

/// Vector multiplication by M4 with assignment.
impl std::ops::MulAssign<M4> for V4 {
    fn mul_assign(&mut self, rhs: M4) {
        *self = *self * rhs;
    }
}

/// Vector division by scalar.
impl std::ops::Div<f32> for V4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * rhs.recip()
    }
}

/// Vector division by scalar with assignment.
impl std::ops::DivAssign<f32> for V4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs
    }
}

/// Vector comparison by magnitude.
impl std::cmp::PartialOrd for V4 {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.magnitude_squared().partial_cmp(&rhs.magnitude_squared())
    }
}

/// Vector magnitude.
impl Magnitude for V4 {
    fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

/// Vector normalization.
impl Normalize for V4 {
    fn normalize(&self) -> Self {
        *self / self.magnitude()
    }
}

impl V4 {
    /// Creates a new [`V4`] using the provided values.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Calculates dot product of two vectors.
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    /// Calculates the Ochiai measure for two vectors.
    pub fn similarity(&self, other: &Self) -> f32 {
        (self.dot(*other)) / f32::sqrt(self.magnitude() * other.magnitude())
    }

    /// Create a [`V3`], dropping the `w` value.
    pub fn truncate(self) -> V3 {
        V3::new(self.x, self.y, self.z)
    }
}
