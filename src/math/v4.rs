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

impl Bounded for V4 {
    const MIN_BOUND: V4 = V4::new(0.0, 0.0, 0.0, 0.0);
    const MAX_BOUND: V4 = V4::new(f32::MAX_BOUND, f32::MAX_BOUND, f32::MAX_BOUND, f32::MAX_BOUND);
}

impl One for V4 {
    fn one() -> Self {
        Self {
            x: 0.5,
            y: 0.5,
            z: 0.5,
            w: 0.5,
        }
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

/// Vector ⋅ scalar addition.
///
/// # $(0, 0, 0, 0) + a$
///
/// To add a scalar to a zero vector, we’ll create new vector with equal components and with magnitude equal to given scalar.
///
/// If a scalar is negative, resulting vector components will be negative.
///
/// $(0, 0, 0, 0) + a = (\frac{a}{2}, \frac{a}{2}, \frac{a}{2}, \frac{a}{2})$
///
/// # $(x, y, z, w) + a$
///
/// To add a scalar to a non-zero vector, we’ll change it to make its magnitude equal $|(x, y, z, w)| + a$.
///
/// When a scalar is negative and its absolute value is greater than vector magnitude, resulting vector will be antiparallel to an original one.
///
/// $(x, y, z, w) + a = (\frac{x (l + a)}{l}, \frac{y (l + a)}{l}, \frac{z (l + a)}{l}, \frac{w (l + a)}{l})$, where $l = |(x, y, z, w)|$.
impl std::ops::Add<f32> for V4 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let mag = self.magnitude();

        if mag == 0.0 {
            Self {
                x: rhs * 0.5,
                y: rhs * 0.5,
                z: rhs * 0.5,
                w: rhs * 0.5,
            }
        } else {
            self * (mag + rhs) / mag
        }
    }
}

/// Vector ⋅ scalar addition with assignment.
impl std::ops::AddAssign<f32> for V4 {
    fn add_assign(&mut self, rhs: f32) {
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

/// Vector ⋅ scalar subtraction.
impl std::ops::Sub<f32> for V4 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        self + -rhs
    }
}

/// Vector ⋅ scalar subtraction with assignment.
impl std::ops::SubAssign<f32> for V4 {
    fn sub_assign(&mut self, rhs: f32) {
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
        self * rhs.inv()
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
        self.magnitude_proportional().partial_cmp(&rhs.magnitude_proportional())
    }
}

/// Vector magnitude.
impl Magnitude for V4 {
    type Output = f32;

    fn magnitude_proportional(self) -> Self::Output {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    fn magnitude(self) -> Self::Output {
        f32::sqrt(self.magnitude_proportional())
    }
}

/// Numeric value of vector.
impl ToNumeric<f32> for V4 {
    fn into_numeric_proportional(self) -> f32 {
        self.magnitude_proportional()
    }

    fn into_numeric(self) -> f32 {
        self.magnitude()
    }
}

/// Vector normalization.
impl Normalize for V4 {
    fn normalize(self) -> Self {
        let mag = self.magnitude();

        self / if mag == 0.0 { 1.0 } else { mag }
    }
}

/// Vector division by `usize`.
impl DivUsize for V4 {
    fn div_usize(self, rhs: usize) -> Self {
        self / rhs as f32
    }
}

/// Dot product.
impl Dot for V4 {
    type Output = f32;

    fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

/// Ochiai measure for two vectors.
impl Similarity for V4 {
    type Output = f32;

    fn similarity(self, other: Self) -> Self::Output {
        self.dot(other) / f32::sqrt(self.magnitude() * other.magnitude())
    }
}

/// Entrywise vector ⋅ scalar addition.
impl EntrywiseAdd<f32> for V4 {
    type Output = Self;

    fn entrywise_add(self, rhs: f32) -> Self {
        Self::new(self.x + rhs, self.y + rhs, self.z + rhs, self.w + rhs)
    }
}

/// Entrywise vector ⋅ scalar subtraction.
impl EntrywiseSub<f32> for V4 {
    type Output = Self;

    fn entrywise_sub(self, rhs: f32) -> Self {
        Self::new(self.x - rhs, self.y - rhs, self.z - rhs, self.w - rhs)
    }
}

/// Entrywise vector ⋅ vector multiplication; i.e., Hadamard product.
impl EntrywiseMul for V4 {
    type Output = Self;

    fn entrywise_mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z, self.w * rhs.w)
    }
}

/// Entrywise vector ⋅ vector division.
impl EntrywiseDiv for V4 {
    type Output = Self;

    fn entrywise_div(self, rhs: Self) -> Self {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
    }
}

/// Entrywise vector ⋅ vector exponentiation.
impl EntrywisePow for V4 {
    type Output = Self;

    fn entrywise_pow(self, rhs: Self) -> Self {
        Self::new(self.x.power(rhs.x), self.y.power(rhs.y), self.z.power(rhs.z), self.w.power(rhs.w))
    }
}

/// Entrywise vector ⋅ scalar exponentiation.
impl EntrywisePow<f32> for V4 {
    type Output = Self;

    fn entrywise_pow(self, rhs: f32) -> Self {
        Self::new(self.x.power(rhs), self.y.power(rhs), self.z.power(rhs), self.w.power(rhs))
    }
}

/// Entrywise vector square root.
impl EntrywiseSqrt for V4 {
    type Output = Self;

    fn entrywise_sqrt(self) -> Self {
        Self::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt(), self.w.sqrt())
    }
}

/// Entrywise vector inversion.
impl EntrywiseInv for V4 {
    type Output = Self;

    fn entrywise_inv(self) -> Self {
        Self::new(self.x.inv(), self.y.inv(), self.z.inv(), self.w.inv())
    }
}

/// Entrywise vector exponentiation.
impl EntrywiseEX for V4 {
    type Output = Self;

    fn entrywise_e_x(self) -> Self::Output {
        Self {
            x: self.x.e_x(),
            y: self.y.e_x(),
            z: self.z.e_x(),
            w: self.w.e_x(),
        }
    }
}

impl V4 {
    /// Creates a new [`V4`] using the provided values.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Create a [`V3`], dropping the `w` value.
    pub fn truncate(self) -> V3 {
        V3::new(self.x, self.y, self.z)
    }
}
