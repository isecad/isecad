#![allow(dead_code)]

use crate::*;

/// A 3×3 matrix; aligned to 64 bytes to be binary compatible with the [`M4`].
#[derive(Debug, Copy, Clone, Default)]
#[repr(align(64))]
pub struct M3 {
    pub a: V3,
    pub b: V3,
    pub c: V3,
}

impl M3 {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32) -> M3 {
        M3 {
            a: V3::new(a, b, c),
            b: V3::new(d, e, f),
            c: V3::new(g, h, i),
        }
    }

    pub fn identity() -> M3 {
        M3 {
            a: V3::unit_x(),
            b: V3::unit_y(),
            c: V3::unit_z(),
        }
    }

    pub fn invert(&self, result: &mut M3) {
        let a: f32 = self.a.x;
        let b: f32 = self.a.y;
        let c: f32 = self.a.z;
        let d: f32 = self.b.x;
        let e: f32 = self.b.y;
        let f: f32 = self.b.z;
        let g: f32 = self.c.x;
        let h: f32 = self.c.y;
        let i: f32 = self.c.z;

        let ei_m_fh: f32 = e * i - f * h;
        let di: f32 = d * i;
        let fg: f32 = f * g;
        let dh_m_eg: f32 = d * h - e * g;

        let inv_det: f32 = 1.0 / (a * ei_m_fh - b * (di - fg) + c * dh_m_eg);

        result.a.x = ei_m_fh * inv_det;
        result.a.y = (c * h - b * i) * inv_det;
        result.a.z = (b * f - c * e) * inv_det;

        result.b.x = (fg - di) * inv_det;
        result.b.y = (a * i - c * g) * inv_det;
        result.b.z = (c * d - a * f) * inv_det;

        result.c.x = dh_m_eg * inv_det;
        result.c.y = (g * b - a * h) * inv_det;
        result.c.z = (a * e - d * b) * inv_det;
    }

    pub fn mul_m3(&mut self, other: &M3) {
        let a: f32 = self.a.x;
        let b: f32 = self.a.y;
        let c: f32 = self.a.z;
        let d: f32 = self.b.x;
        let e: f32 = self.b.y;
        let f: f32 = self.b.z;
        let g: f32 = self.c.x;
        let h: f32 = self.c.y;
        let i: f32 = self.c.z;

        let α: f32 = other.a.x;
        let β: f32 = other.a.y;
        let γ: f32 = other.a.z;

        self.a.x = α * a + β * d + γ * g;
        self.a.y = α * b + β * e + γ * h;
        self.a.z = α * c + β * f + γ * i;

        let α: f32 = other.b.x;
        let β: f32 = other.b.y;
        let γ: f32 = other.b.z;

        self.b.x = α * a + β * d + γ * g;
        self.b.y = α * b + β * e + γ * h;
        self.b.z = α * c + β * f + γ * i;

        let α: f32 = other.c.x;
        let β: f32 = other.c.y;
        let γ: f32 = other.c.z;

        self.c.x = α * a + β * d + γ * g;
        self.c.y = α * b + β * e + γ * h;
        self.c.z = α * c + β * f + γ * i;
    }
}
