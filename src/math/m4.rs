#![allow(dead_code)]

use crate::*;

/// A 4×4 matrix.
///
/// We will use column-major order, so vectors are columns.
///
/// Aligned to 64 bytes to match a cache line.
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(align(64))]
pub struct M4 {
    pub a: V4,
    pub b: V4,
    pub c: V4,
    pub d: V4,
}

impl Default for M4 {
    fn default() -> Self {
        Self {
            a: V4::new(1.0, 0.0, 0.0, 0.0),
            b: V4::new(0.0, 1.0, 0.0, 0.0),
            c: V4::new(0.0, 0.0, 1.0, 0.0),
            d: V4::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl From<[[f32; 4]; 4]> for M4 {
    fn from([a, b, c, d]: [[f32; 4]; 4]) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
            c: c.into(),
            d: d.into(),
        }
    }
}

impl Into<[[f32; 4]; 4]> for M4 {
    fn into(self) -> [[f32; 4]; 4] {
        [self.a.into(), self.b.into(), self.c.into(), self.d.into()]
    }
}

/// Matrix negation.
impl std::ops::Neg for M4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            a: -self.a,
            b: -self.b,
            c: -self.c,
            d: -self.d,
        }
    }
}

/// Matrix addition.
impl std::ops::Add for M4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
            d: self.d + rhs.d,
        }
    }
}

/// Matrix addition with assignment.
impl std::ops::AddAssign for M4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Matrix subtraction.
impl std::ops::Sub for M4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
            c: self.c - rhs.c,
            d: self.d - rhs.d,
        }
    }
}

/// Matrix subtraction with assignment.
impl std::ops::SubAssign for M4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Matrix multiplication by scalar.
impl std::ops::Mul<f32> for M4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
            d: self.d * rhs,
        }
    }
}

/// Matrix multiplication by scalar with assignment.
impl std::ops::MulAssign<f32> for M4 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

/// Matrix multiplication.
impl std::ops::Mul for M4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x = V4::new(self.a.x, self.b.x, self.c.x, self.d.x);
        let y = V4::new(self.a.y, self.b.y, self.c.y, self.d.y);
        let z = V4::new(self.a.z, self.b.z, self.c.z, self.d.z);
        let w = V4::new(self.a.w, self.b.w, self.c.w, self.d.w);

        Self {
            a: V4::new(rhs.a.dot(x), rhs.a.dot(y), rhs.a.dot(z), rhs.a.dot(w)),
            b: V4::new(rhs.b.dot(x), rhs.b.dot(y), rhs.b.dot(z), rhs.b.dot(w)),
            c: V4::new(rhs.c.dot(x), rhs.c.dot(y), rhs.c.dot(z), rhs.c.dot(w)),
            d: V4::new(rhs.d.dot(x), rhs.d.dot(y), rhs.d.dot(z), rhs.d.dot(w)),
        }
    }
}

/// Matrix multiplication with assignment.
impl std::ops::MulAssign for M4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

/// Matrix division by scalar.
impl std::ops::Div<f32> for M4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * rhs.inv()
    }
}

/// Matrix division by scalar with assignment.
impl std::ops::DivAssign<f32> for M4 {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}

/// Matrix transposition.
impl Transpose for M4 {
    fn transpose(&self) -> Self {
        Self {
            a: V4::new(self.a.x, self.b.x, self.c.x, self.d.x),
            b: V4::new(self.a.y, self.b.y, self.c.y, self.d.y),
            c: V4::new(self.a.z, self.b.z, self.c.z, self.d.z),
            d: V4::new(self.a.w, self.b.w, self.c.w, self.d.w),
        }
    }
}

/// Matrix inversion.
impl Inv for M4 {
    fn inv(self) -> Self {
        let Self {
            a: V4 { x: a, y: b, z: c, w: d },
            b: V4 { x: e, y: f, z: g, w: h },
            c: V4 { x: i, y: j, z: k, w: l },
            d: V4 { x: m, y: n, z: o, w: p },
        } = self;

        let m_af = k * p - l * o;
        let m_ah = j * o - k * n;
        let m_bg = i * p - l * m;
        let m_ce = j * p - l * n;
        let m_ch = i * n - j * m;
        let m_df = i * o - k * m;

        let m_in = c * h - d * g;
        let m_ip = b * g - c * f;
        let m_jo = a * h - d * e;
        let m_km = b * h - d * f;
        let m_kp = a * f - b * e;
        let m_ln = a * g - c * e;

        let m_a = f * m_af - g * m_ce + h * m_ah;
        let m_b = g * m_bg - e * m_af - h * m_df;
        let m_c = e * m_ce - f * m_bg + h * m_ch;
        let m_d = f * m_df - e * m_ah - g * m_ch;

        let m_e = c * m_ce - b * m_af - d * m_ah;
        let m_f = a * m_af - c * m_bg + d * m_df;
        let m_g = b * m_bg - a * m_ce - d * m_ch;
        let m_h = a * m_ah - b * m_df + c * m_ch;

        let m_i = n * m_in - o * m_km + p * m_ip;
        let m_j = o * m_jo - m * m_in - p * m_ln;
        let m_k = m * m_km - n * m_jo + p * m_kp;
        let m_l = n * m_ln - m * m_ip - o * m_kp;

        let m_m = k * m_km - j * m_in - l * m_ip;
        let m_n = i * m_in - k * m_jo + l * m_ln;
        let m_o = j * m_jo - i * m_km - l * m_kp;
        let m_p = i * m_ip - j * m_ln + k * m_kp;

        let det = a * m_a + b * m_b + c * m_c + d * m_d;

        Self {
            a: V4::new(m_a, m_e, m_i, m_m),
            b: V4::new(m_b, m_f, m_j, m_n),
            c: V4::new(m_c, m_g, m_k, m_o),
            d: V4::new(m_d, m_h, m_l, m_p),
        } / det
    }
}

impl M4 {
    /// Converts a transform matrix to a new translation vector.
    pub fn to_translation_v3(&self) -> V3 {
        self.d.truncate()
    }
}
