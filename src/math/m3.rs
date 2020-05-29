#![allow(dead_code)]

use crate::*;

/// A 3×3 matrix; aligned to 64 bytes to be binary compatible with the [`M4`].
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct M3 {
    pub a: V3,
    pub b: V3,
    pub c: V3,
}

impl Default for M3 {
    fn default() -> Self {
        Self::identity()
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

impl M3 {
    pub const fn identity() -> Self {
        Self {
            a: V3::unit_x(),
            b: V3::unit_y(),
            c: V3::unit_z(),
        }
    }

    pub fn invert(&self, output: &mut M3) {
        let Self { a, b, c } = *self;

        let ei_m_fh = b.y * c.z - b.z * c.y;
        let di = b.x * c.z;
        let fg = b.z * c.x;
        let dh_m_eg = b.x * c.y - b.y * c.x;

        let inv_det = (a.x * ei_m_fh - a.y * (di - fg) + a.z * dh_m_eg).recip();

        output.a.x = ei_m_fh * inv_det;
        output.a.y = (a.z * c.y - a.y * c.z) * inv_det;
        output.a.z = (a.y * b.z - a.z * b.y) * inv_det;

        output.b.x = (fg - di) * inv_det;
        output.b.y = (a.x * c.z - a.z * c.x) * inv_det;
        output.b.z = (a.z * b.x - a.x * b.z) * inv_det;

        output.c.x = dh_m_eg * inv_det;
        output.c.y = (c.x * a.y - a.x * c.y) * inv_det;
        output.c.z = (a.x * b.y - b.x * a.y) * inv_det;
    }

    pub fn mul_m3(&mut self, other: &Self) {
        let x = V3::new(self.a.x, self.b.x, self.c.x);
        let y = V3::new(self.a.y, self.b.y, self.c.y);
        let z = V3::new(self.a.z, self.b.z, self.c.z);

        self.a = V3::new(other.a.dot(x), other.a.dot(y), other.a.dot(z));
        self.b = V3::new(other.b.dot(x), other.b.dot(y), other.b.dot(z));
        self.c = V3::new(other.c.dot(x), other.c.dot(y), other.c.dot(z));
    }
}
