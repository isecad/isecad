#![allow(dead_code)]

use crate::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct M4 {
    pub a: V4,
    pub b: V4,
    pub c: V4,
    pub d: V4,
}

impl M4 {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32, j: f32, k: f32, l: f32, m: f32, n: f32, o: f32, p: f32) -> M4 {
        M4 {
            a: V4::new(a, b, c, d),
            b: V4::new(e, f, g, h),
            c: V4::new(i, j, k, l),
            d: V4::new(m, n, o, p),
        }
    }

    pub fn identity() -> M4 {
        M4 {
            a: V4::unit_x(),
            b: V4::unit_y(),
            c: V4::unit_z(),
            d: V4::unit_w(),
        }
    }

    pub fn invert(&self, result: &mut M4) {
        let a: f32 = self.a.x;
        let b: f32 = self.a.y;
        let c: f32 = self.a.z;
        let d: f32 = self.a.w;

        let e: f32 = self.b.x;
        let f: f32 = self.b.y;
        let g: f32 = self.b.z;
        let h: f32 = self.b.w;

        let i: f32 = self.c.x;
        let j: f32 = self.c.y;
        let k: f32 = self.c.z;
        let l: f32 = self.c.w;

        let m: f32 = self.d.x;
        let n: f32 = self.d.y;
        let o: f32 = self.d.z;
        let p: f32 = self.d.w;

        let α: f32 = a * f - b * e;
        let β: f32 = a * g - c * e;
        let γ: f32 = a * h - d * e;
        let δ: f32 = b * g - c * f;
        let ε: f32 = b * h - d * f;
        let ζ: f32 = c * h - d * g;
        let η: f32 = i * n - j * m;
        let θ: f32 = i * o - k * m;
        let ι: f32 = i * p - l * m;
        let κ: f32 = j * o - k * n;
        let λ: f32 = j * p - l * n;
        let μ: f32 = k * p - l * o;

        let inv_det: f32 = 1.0 / (α * μ - β * λ + γ * κ + δ * ι - ε * θ + ζ * η);

        result.a.x = (f * μ - g * λ + h * κ) * inv_det;
        result.a.y = (c * λ - b * μ - d * κ) * inv_det;
        result.a.z = (n * ζ - o * ε + p * δ) * inv_det;
        result.a.w = (k * ε - j * ζ - l * δ) * inv_det;

        result.b.x = (g * ι - e * μ - h * θ) * inv_det;
        result.b.y = (a * μ - c * ι + d * θ) * inv_det;
        result.b.z = (o * γ - m * ζ - p * β) * inv_det;
        result.b.w = (i * ζ - k * γ + l * β) * inv_det;

        result.c.x = (e * λ - f * ι + h * η) * inv_det;
        result.c.y = (b * ι - a * λ - d * η) * inv_det;
        result.c.z = (m * ε - n * γ + p * α) * inv_det;
        result.c.w = (j * γ - i * ε - l * α) * inv_det;

        result.d.x = (f * θ - e * κ - g * η) * inv_det;
        result.d.y = (a * κ - b * θ + c * η) * inv_det;
        result.d.z = (n * β - m * δ - o * α) * inv_det;
        result.d.w = (i * δ - j * β + k * α) * inv_det;
    }

    pub fn mul_m4(&mut self, other: &M4) {
        let a: f32 = self.a.x;
        let b: f32 = self.a.y;
        let c: f32 = self.a.z;
        let d: f32 = self.a.w;

        let e: f32 = self.b.x;
        let f: f32 = self.b.y;
        let g: f32 = self.b.z;
        let h: f32 = self.b.w;

        let i: f32 = self.c.x;
        let j: f32 = self.c.y;
        let k: f32 = self.c.z;
        let l: f32 = self.c.w;

        let m: f32 = self.d.x;
        let n: f32 = self.d.y;
        let o: f32 = self.d.z;
        let p: f32 = self.d.w;

        let α: f32 = other.a.x;
        let β: f32 = other.a.y;
        let γ: f32 = other.a.z;
        let δ: f32 = other.a.w;

        self.a.x = α * a + β * e + γ * i + δ * m;
        self.a.y = α * b + β * f + γ * j + δ * n;
        self.a.z = α * c + β * g + γ * k + δ * o;
        self.a.w = α * d + β * h + γ * l + δ * p;

        let α: f32 = other.b.x;
        let β: f32 = other.b.y;
        let γ: f32 = other.b.z;
        let δ: f32 = other.b.w;

        self.b.x = α * a + β * e + γ * i + δ * m;
        self.b.y = α * b + β * f + γ * j + δ * n;
        self.b.z = α * c + β * g + γ * k + δ * o;
        self.b.w = α * d + β * h + γ * l + δ * p;

        let α: f32 = other.c.x;
        let β: f32 = other.c.y;
        let γ: f32 = other.c.z;
        let δ: f32 = other.c.w;

        self.c.x = α * a + β * e + γ * i + δ * m;
        self.c.y = α * b + β * f + γ * j + δ * n;
        self.c.z = α * c + β * g + γ * k + δ * o;
        self.c.w = α * d + β * h + γ * l + δ * p;

        let α: f32 = other.d.x;
        let β: f32 = other.d.y;
        let γ: f32 = other.d.z;
        let δ: f32 = other.d.w;

        self.d.x = α * a + β * e + γ * i + δ * m;
        self.d.y = α * b + β * f + γ * j + δ * n;
        self.d.z = α * c + β * g + γ * k + δ * o;
        self.d.w = α * d + β * h + γ * l + δ * p;
    }

    pub fn get_translation_v3(&self) -> &V3 {
        let v3 = &self.d as *const V4 as usize as *const V3;

        unsafe { &*v3 }
    }
}
