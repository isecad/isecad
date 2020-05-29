#![allow(dead_code)]

use crate::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct M4 {
    pub a: V4,
    pub b: V4,
    pub c: V4,
    pub d: V4,
}

impl Default for M4 {
    fn default() -> Self {
        Self::identity()
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

impl M4 {
    pub const fn identity() -> Self {
        Self {
            a: V4::unit_x(),
            b: V4::unit_y(),
            c: V4::unit_z(),
            d: V4::unit_w(),
        }
    }

    pub fn invert(&self, output: &mut Self) {
        let Self {
            a: V4 { x: a, y: b, z: c, w: d },
            b: V4 { x: e, y: f, z: g, w: h },
            c: V4 { x: i, y: j, z: k, w: l },
            d: V4 { x: m, y: n, z: o, w: p },
        } = *self;

        let α = a * f - b * e;
        let β = a * g - c * e;
        let γ = a * h - d * e;
        let δ = b * g - c * f;
        let ε = b * h - d * f;
        let ζ = c * h - d * g;
        let η = i * n - j * m;
        let θ = i * o - k * m;
        let ι = i * p - l * m;
        let κ = j * o - k * n;
        let λ = j * p - l * n;
        let μ = k * p - l * o;

        let inv_det = (α * μ - β * λ + γ * κ + δ * ι - ε * θ + ζ * η).recip();

        output.a.x = (f * μ - g * λ + h * κ) * inv_det;
        output.a.y = (c * λ - b * μ - d * κ) * inv_det;
        output.a.z = (n * ζ - o * ε + p * δ) * inv_det;
        output.a.w = (k * ε - j * ζ - l * δ) * inv_det;

        output.b.x = (g * ι - e * μ - h * θ) * inv_det;
        output.b.y = (a * μ - c * ι + d * θ) * inv_det;
        output.b.z = (o * γ - m * ζ - p * β) * inv_det;
        output.b.w = (i * ζ - k * γ + l * β) * inv_det;

        output.c.x = (e * λ - f * ι + h * η) * inv_det;
        output.c.y = (b * ι - a * λ - d * η) * inv_det;
        output.c.z = (m * ε - n * γ + p * α) * inv_det;
        output.c.w = (j * γ - i * ε - l * α) * inv_det;

        output.d.x = (f * θ - e * κ - g * η) * inv_det;
        output.d.y = (a * κ - b * θ + c * η) * inv_det;
        output.d.z = (n * β - m * δ - o * α) * inv_det;
        output.d.w = (i * δ - j * β + k * α) * inv_det;
    }

    pub fn mul_m4(&mut self, other: &Self) {
        let x = V4::new(self.a.x, self.b.x, self.c.x, self.d.x);
        let y = V4::new(self.a.y, self.b.y, self.c.y, self.d.y);
        let z = V4::new(self.a.z, self.b.z, self.c.z, self.d.z);
        let w = V4::new(self.a.w, self.b.w, self.c.w, self.d.w);

        self.a = V4::new(other.a.dot(x), other.a.dot(y), other.a.dot(z), other.a.dot(w));
        self.b = V4::new(other.b.dot(x), other.b.dot(y), other.b.dot(z), other.b.dot(w));
        self.c = V4::new(other.c.dot(x), other.c.dot(y), other.c.dot(z), other.c.dot(w));
        self.d = V4::new(other.d.dot(x), other.d.dot(y), other.d.dot(z), other.d.dot(w));
    }

    pub fn get_translation_v3(&self) -> &V3 {
        unsafe { &*(&self.d as *const V4 as *const V3) }
    }
}
