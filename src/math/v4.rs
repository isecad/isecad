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

impl From<(f32, f32, f32, f32)> for V4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self { x, y, z, w }
    }
}

impl Into<[f32; 4]> for V4 {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

impl Into<(f32, f32, f32, f32)> for V4 {
    fn into(self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }
}

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

impl std::ops::Add for V4 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::AddAssign for V4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Sub for V4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::SubAssign for V4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl std::ops::Mul<f32> for V4 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl std::ops::MulAssign<f32> for V4 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = *self * scalar
    }
}

impl std::ops::Div<f32> for V4 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl std::ops::DivAssign<f32> for V4 {
    fn div_assign(&mut self, scalar: f32) {
        *self = *self / scalar
    }
}

impl V4 {
    /// Construct a new [`V4`], using the provided values.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// A zero vector.
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// A unit vector in the `x` direction.
    pub const fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    /// A unit vector in the `y` direction.
    pub const fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0, 0.0)
    }

    /// A unit vector in the `z` direction.
    pub const fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0)
    }

    /// A unit vector in the `w` direction.
    pub const fn unit_w() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Create a [`V3`], dropping the `w` value.
    pub const fn truncate(self) -> V3 {
        V3::new(self.x, self.y, self.z)
    }

    /// Returns the dot product of the vector and `other`.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}
