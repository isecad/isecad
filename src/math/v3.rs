#![allow(dead_code)]

use crate::*;

/// A 3D vector.
///
/// Aligned to 16 bytes to simplify `v128.load` operations on it, and to be binary compatible with the [`V4`].
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(align(16))]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<[f32; 3]> for V3 {
    fn from([x, y, z]: [f32; 3]) -> Self {
        Self { x, y, z }
    }
}

impl From<(f32, f32, f32)> for V3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        Self { x, y, z }
    }
}

impl Into<[f32; 3]> for V3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl Into<(f32, f32, f32)> for V3 {
    fn into(self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }
}

impl std::ops::Neg for V3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Add for V3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for V3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl std::ops::Sub for V3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign for V3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl std::ops::Mul<f32> for V3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::MulAssign<f32> for V3 {
    fn mul_assign(&mut self, scalar: f32) {
        *self = *self * scalar
    }
}

impl std::ops::Div<f32> for V3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl std::ops::DivAssign<f32> for V3 {
    fn div_assign(&mut self, scalar: f32) {
        *self = *self / scalar
    }
}

impl V3 {
    /// Construct a new [`V3`], using the provided values.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// A zero vector.
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// A unit vector in the `x` direction.
    pub const fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// A unit vector in the `y` direction.
    pub const fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// A unit vector in the `z` direction.
    pub const fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Create a [`V4`], using the `x`, `y` and `z` values from this vector, and the provided `w`.
    pub const fn extend(self, w: f32) -> V4 {
        V4::new(self.x, self.y, self.z, w)
    }

    /// Returns the dot product of the vector and `other`.
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl V3 {
    pub fn reset(&mut self) {
        *self = Self::zero()
    }

    pub fn dot_product(&self, other: &Self) -> f32 {
        self.dot(*other)
    }

    pub fn add_v3(&mut self, other: &Self) {
        *self += *other
    }

    pub fn mul_scalar(&self, scalar: f32, output: &mut Self) {
        *output *= scalar
    }

    pub fn div_scalar(&self, scalar: f32, output: &mut Self) {
        *output *= scalar.recip()
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn magnitude_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn similarity(&self, other: &Self) -> f32 {
        self.dot_product(other) / (self.magnitude() * other.magnitude())
    }

    pub fn normalize(&self, output: &mut Self) {
        *output = *self * self.magnitude().recip()
    }

    pub fn to_rotation_m3(&self, output: &mut M3) {
        let angle = self.magnitude();
        let inv_mag = angle.recip();

        let x = self.x * inv_mag;
        let y = self.y * inv_mag;
        let z = self.z * inv_mag;

        let (sθ, cθ) = angle.sin_cos();
        let vθ = 1.0 - cθ;

        let (xvθ, yvθ) = (x * vθ, y * vθ);
        let (xsθ, ysθ, zsθ) = (x * sθ, y * sθ, z * sθ);
        let (xyvθ, xzvθ, yzvθ) = (y * xvθ, z * xvθ, z * yvθ);

        #[rustfmt::skip]
        {
            output.a.x = cθ + x * xvθ; output.a.y = xyvθ   - zsθ; output.a.z = xzvθ       + ysθ;
            output.b.x = xyvθ   + zsθ; output.b.y = cθ + y * yvθ; output.b.z = yzvθ       - xsθ;
            output.c.x = xzvθ   - ysθ; output.c.y = yzvθ   + xsθ; output.c.z = cθ + z * z * vθ;
        }
    }

    /// Doesn’t reset the result matrix; except its last row, it should be an identity matrix.
    pub fn to_translation_m4(&self, output: &mut M4) {
        output.d.x = self.x;
        output.d.y = self.y;
        output.d.z = self.z;
        output.d.w = 0.0;
    }

    /// The vector should be normalized.
    pub fn to_rotation_m4(&self, angle: f32, output: &mut M4) {
        let x: f32 = self.x;
        let y: f32 = self.y;
        let z: f32 = self.z;

        let cθ: f32 = f32::cos(angle);
        let sθ: f32 = f32::sin(angle);
        let vθ: f32 = 1.0 - cθ;

        let xvθ: f32 = x * vθ;
        let yvθ: f32 = y * vθ;

        let xsθ: f32 = x * sθ;
        let ysθ: f32 = y * sθ;
        let zsθ: f32 = z * sθ;

        let xyvθ: f32 = y * xvθ;
        let xzvθ: f32 = z * xvθ;
        let yzvθ: f32 = z * yvθ;

        #[rustfmt::skip]
        {
            output.a.x = cθ + x * xvθ; output.a.y = xyvθ   - zsθ; output.a.z = xzvθ       + ysθ; output.a.w =  0.0;
            output.b.x = xyvθ   + zsθ; output.b.y = cθ + y * yvθ; output.b.z = yzvθ       - xsθ; output.b.w =  0.0;
            output.c.x = xzvθ   - ysθ; output.c.y = yzvθ   + xsθ; output.c.z = cθ + z * z * vθ;  output.c.w =  0.0;
            output.d.x = 0.0;          output.d.y = 0.0;          output.d.z = 0.0;              output.d.w =  1.0;
        }
    }
}
