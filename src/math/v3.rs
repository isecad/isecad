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

impl Into<[f32; 3]> for V3 {
    fn into(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

/// Vector negation.
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

/// Vector addition.
impl std::ops::Add for V3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Vector addition with assignment.
impl std::ops::AddAssign for V3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Vector subtraction.
impl std::ops::Sub for V3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Vector subtraction with assignment.
impl std::ops::SubAssign for V3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Vector multiplication by scalar.
impl std::ops::Mul<f32> for V3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Vector multiplication by scalar with assignment.
impl std::ops::MulAssign<f32> for V3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

/// Vector multiplication by M3.
impl std::ops::Mul<M3> for V3 {
    type Output = Self;

    fn mul(self, rhs: M3) -> Self::Output {
        let Self { x, y, z } = self;

        Self {
            x: rhs.a.x * x + rhs.b.x * y + rhs.c.x * z,
            y: rhs.a.y * x + rhs.b.y * y + rhs.c.y * z,
            z: rhs.a.z * x + rhs.b.z * y + rhs.c.z * z,
        }
    }
}

/// Vector multiplication by M3 with assignment.
impl std::ops::MulAssign<M3> for V3 {
    fn mul_assign(&mut self, rhs: M3) {
        *self = *self * rhs;
    }
}

/// Vector multiplication by M4.
impl std::ops::Mul<M4> for V3 {
    type Output = Self;

    fn mul(self, rhs: M4) -> Self::Output {
        let Self { x, y, z } = self;

        Self {
            x: rhs.a.x * x + rhs.b.x * y + rhs.c.x * z + rhs.d.x,
            y: rhs.a.y * x + rhs.b.y * y + rhs.c.y * z + rhs.d.y,
            z: rhs.a.z * x + rhs.b.z * y + rhs.c.z * z + rhs.d.z,
        }
    }
}

/// Vector multiplication by M4 with assignment.
impl std::ops::MulAssign<M4> for V3 {
    fn mul_assign(&mut self, rhs: M4) {
        *self = *self * rhs;
    }
}

/// Vector division by scalar.
impl std::ops::Div<f32> for V3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * rhs.inv()
    }
}

/// Vector division by scalar with assignment.
impl std::ops::DivAssign<f32> for V3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

/// Vector comparison by magnitude.
impl std::cmp::PartialOrd for V3 {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.magnitude_squared().partial_cmp(&rhs.magnitude_squared())
    }
}

/// Vector magnitude.
impl Magnitude for V3 {
    fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

/// Vector normalization.
impl Normalize for V3 {
    fn normalize(&self) -> Self {
        *self / self.magnitude()
    }
}

impl V3 {
    /// Creates a new [`V3`] using the provided values.
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Calculates cross product of two vectors.
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Calculates dot product of two vectors.
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Calculates the Ochiai measure for two vectors.
    pub fn similarity(&self, other: &Self) -> f32 {
        (self.dot(*other)) / f32::sqrt(self.magnitude() * other.magnitude())
    }

    /// Create a [`V4`], using the `x`, `y` and `z` values from this vector, and the provided `w`.
    pub fn extend(self, w: f32) -> V4 {
        V4::new(self.x, self.y, self.z, w)
    }

    /// Converts a rotation vector to a new 3×3 transform matrix.
    pub fn to_rotation_m3(&self) -> M3 {
        let θ = self.magnitude();
        let V3 { x, y, z } = *self / θ;

        let (sθ, cθ) = θ.sin_cos();
        let vθ = 1.0 - cθ;

        let xvθ = x * vθ;
        let yvθ = y * vθ;

        let xsθ = x * sθ;
        let ysθ = y * sθ;
        let zsθ = z * sθ;

        let xyvθ = y * xvθ;
        let xzvθ = z * xvθ;
        let yzvθ = z * yvθ;

        #[rustfmt::skip]
        M3 {
            a: V3::new(cθ + x * xvθ, xyvθ + zsθ,   xzvθ - ysθ),
            b: V3::new(xyvθ - zsθ,   cθ + y * yvθ, yzvθ + xsθ),
            c: V3::new(xzvθ + ysθ,   yzvθ - xsθ,   cθ + z * z * vθ),
        }
    }

    /// Converts a rotation vector to a new 4×4 transform matrix.
    pub fn to_rotation_m4(&self) -> M4 {
        let θ = self.magnitude();
        let V3 { x, y, z } = *self / θ;

        let (sθ, cθ) = θ.sin_cos();
        let vθ = 1.0 - cθ;

        let xvθ = x * vθ;
        let yvθ = y * vθ;

        let xsθ = x * sθ;
        let ysθ = y * sθ;
        let zsθ = z * sθ;

        let xyvθ = y * xvθ;
        let xzvθ = z * xvθ;
        let yzvθ = z * yvθ;

        #[rustfmt::skip]
        M4 {
            a: V4::new(cθ + x * xvθ, xyvθ + zsθ,   xzvθ - ysθ,      0.0),
            b: V4::new(xyvθ - zsθ,   cθ + y * yvθ, yzvθ + xsθ,      0.0),
            c: V4::new(xzvθ + ysθ,   yzvθ - xsθ,   cθ + z * z * vθ, 0.0),
            d: V4::new(0.0,          0.0,          0.0,             1.0),
        }
    }

    /// Converts a translation vector to a new 4×4 transform matrix.
    pub fn to_translation_m4(&self) -> M4 {
        M4 {
            a: V4::new(1.0, 0.0, 0.0, 0.0),
            b: V4::new(0.0, 1.0, 0.0, 0.0),
            c: V4::new(0.0, 0.0, 1.0, 0.0),
            d: self.extend(1.0),
        }
    }
}
