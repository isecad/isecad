#![allow(dead_code)]

use crate::*;

/// A 3×3 matrix.
///
/// We will use column-major order, so vectors are columns.
///
/// Aligned to 64 bytes to match a cache line.
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(align(64))]
pub struct M3 {
    pub a: V3,
    pub b: V3,
    pub c: V3,
}

impl Default for M3 {
    fn default() -> Self {
        Self {
            a: V3::new(1.0, 0.0, 0.0),
            b: V3::new(0.0, 1.0, 0.0),
            c: V3::new(0.0, 0.0, 1.0),
        }
    }
}

impl From<[[f32; 3]; 3]> for M3 {
    fn from([a, b, c]: [[f32; 3]; 3]) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }
}

impl Into<[[f32; 3]; 3]> for M3 {
    fn into(self) -> [[f32; 3]; 3] {
        [self.a.into(), self.b.into(), self.c.into()]
    }
}

/// Matrix negation.
impl std::ops::Neg for M3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            a: -self.a,
            b: -self.b,
            c: -self.c,
        }
    }
}

/// Matrix addition.
impl std::ops::Add for M3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
        }
    }
}

/// Matrix addition with assignment.
impl std::ops::AddAssign for M3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

/// Matrix subtraction.
impl std::ops::Sub for M3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
            c: self.c - rhs.c,
        }
    }
}

/// Matrix subtraction with assignment.
impl std::ops::SubAssign for M3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

/// Matrix multiplication by scalar.
impl std::ops::Mul<f32> for M3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
        }
    }
}

/// Matrix multiplication by scalar with assignment.
impl std::ops::MulAssign<f32> for M3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs
    }
}

/// Matrix multiplication.
impl std::ops::Mul for M3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = V3::new(self.a.x, self.b.x, self.c.x);
        let y = V3::new(self.a.y, self.b.y, self.c.y);
        let z = V3::new(self.a.z, self.b.z, self.c.z);

        Self {
            a: V3::new(rhs.a.dot(x), rhs.a.dot(y), rhs.a.dot(z)),
            b: V3::new(rhs.b.dot(x), rhs.b.dot(y), rhs.b.dot(z)),
            c: V3::new(rhs.c.dot(x), rhs.c.dot(y), rhs.c.dot(z)),
        }
    }
}

/// Matrix multiplication with assignment.
impl std::ops::MulAssign for M3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

/// Matrix division by scalar.
impl std::ops::Div<f32> for M3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * rhs.inv()
    }
}

/// Matrix division by scalar with assignment.
impl std::ops::DivAssign<f32> for M3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs
    }
}

/// Matrix transposition.
impl Transpose for M3 {
    fn transpose(&self) -> Self {
        Self {
            a: V3::new(self.a.x, self.b.x, self.c.x),
            b: V3::new(self.a.y, self.b.y, self.c.y),
            c: V3::new(self.a.z, self.b.z, self.c.z),
        }
    }
}

/// Matrix inversion.
impl Inv for M3 {
    fn inv(self) -> Self {
        let Self {
            a: V3 { x: a, y: b, z: c },
            b: V3 { x: d, y: e, z: f },
            c: V3 { x: g, y: h, z: i },
        } = self;

        let m_a = e * i - h * f;
        let m_b = f * g - d * i;
        let m_c = d * h - e * g;

        let m_d = c * h - b * i;
        let m_e = a * i - c * g;
        let m_f = b * g - a * h;

        let m_g = b * f - c * e;
        let m_h = c * d - a * f;
        let m_i = a * e - b * d;

        let det = a * m_a + b * m_b + c * m_c;

        Self {
            a: V3::new(m_a, m_d, m_g),
            b: V3::new(m_b, m_e, m_h),
            c: V3::new(m_c, m_f, m_i),
        } / det
    }
}
