#![allow(dead_code)]

use crate::*;

/// A 3D vector; aligned to 16 bytes to simplify `v128.load` operations on it, and to be binary compatible with the [`V4`].
#[derive(Debug, Copy, Clone, Default)]
#[repr(align(16))]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl V3 {
    pub const UNIT_X: V3 = V3 { x: 1.0, y: 0.0, z: 0.0 };
    pub const UNIT_Y: V3 = V3 { x: 0.0, y: 1.0, z: 0.0 };
    pub const UNIT_Z: V3 = V3 { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> V3 {
        V3 { x, y, z }
    }

    pub fn unit_x() -> V3 {
        V3 { x: 1.0, y: 0.0, z: 0.0 }
    }

    pub fn unit_y() -> V3 {
        V3 { x: 0.0, y: 1.0, z: 0.0 }
    }

    pub fn unit_z() -> V3 {
        V3 { x: 0.0, y: 0.0, z: 1.0 }
    }

    pub fn reset(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
        self.z = 0.0;
    }

    pub fn dot_product(&self, other: &V3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn add_v3(&mut self, other: &V3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn mul_scalar(&self, scalar: f32, result: &mut V3) {
        result.x = self.x * scalar;
        result.y = self.y * scalar;
        result.z = self.z * scalar;
    }

    pub fn div_scalar(&self, scalar: f32, result: &mut V3) {
        let inv_scalar: f32 = 1.0 / scalar;

        result.x = self.x * inv_scalar;
        result.y = self.y * inv_scalar;
        result.z = self.z * inv_scalar;
    }

    pub fn magnitude(&self) -> f32 {
        f32::sqrt(V3::dot_product(self, self))
    }

    pub fn similarity(&self, other: &V3) -> f32 {
        V3::dot_product(self, other) / (self.magnitude() * other.magnitude())
    }

    pub fn normalize(&self, result: &mut V3) {
        let x: f32 = self.x;
        let y: f32 = self.y;
        let z: f32 = self.z;

        let inv_mag: f32 = 1.0 / f32::sqrt(x * x + y * y + z * z);

        result.x = x * inv_mag;
        result.y = y * inv_mag;
        result.z = z * inv_mag;
    }

    pub fn to_rotation_m3(&self, result: &mut M3) {
        let angle: f32 = self.magnitude();
        let inv_mag: f32 = 1.0 / angle;

        let x: f32 = self.x * inv_mag;
        let y: f32 = self.y * inv_mag;
        let z: f32 = self.z * inv_mag;

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
            result.a.x = cθ + x * xvθ; result.a.y = xyvθ   - zsθ; result.a.z = xzvθ       + ysθ;
            result.b.x = xyvθ   + zsθ; result.b.y = cθ + y * yvθ; result.b.z = yzvθ       - xsθ;
            result.c.x = xzvθ   - ysθ; result.c.y = yzvθ   + xsθ; result.c.z = cθ + z * z * vθ;
        }
    }

    /// Doesn’t reset the result matrix; except its last row, it should be an identity matrix.
    pub fn to_translation_m4(&self, result: &mut M4) {
        result.d.x = self.x;
        result.d.y = self.y;
        result.d.z = self.z;
        result.d.w = 0.0;
    }

    /// The vector should be normalized.
    pub fn to_rotation_m4(&self, angle: f32, result: &mut M4) {
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
            result.a.x = cθ + x * xvθ; result.a.y = xyvθ   - zsθ; result.a.z = xzvθ       + ysθ; result.a.w =  0.0;
            result.b.x = xyvθ   + zsθ; result.b.y = cθ + y * yvθ; result.b.z = yzvθ       - xsθ; result.b.w =  0.0;
            result.c.x = xzvθ   - ysθ; result.c.y = yzvθ   + xsθ; result.c.z = cθ + z * z * vθ;  result.c.w =  0.0;
            result.d.x = 0.0;          result.d.y = 0.0;          result.d.z = 0.0;              result.d.w =  1.0;
        }
    }
}
