#![allow(dead_code)]

#[derive(Debug, Copy, Clone)]
pub struct V4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl V4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> V4 {
        V4 { x, y, z, w }
    }

    pub fn unit_x() -> V4 {
        V4 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn unit_y() -> V4 {
        V4 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn unit_z() -> V4 {
        V4 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        }
    }

    pub fn unit_w() -> V4 {
        V4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn empty() -> V4 {
        V4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    // pub fn reset(&mut self) {
    //     self.x = 0.0;
    //     self.y = 0.0;
    //     self.z = 0.0;
    //     self.w = 0.0;
    // }
    //
    // pub fn dot_product(&self, other: &V4) -> f32 {
    //     self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    // }
    //
    // pub fn add_v4(&mut self, other: &V4) {
    //     self.x += other.x;
    //     self.y += other.y;
    //     self.z += other.z;
    //     self.w += other.w;
    // }
    //
    // pub fn mul_scalar(&self, scalar: f32, result: &mut V4) {
    //     result.x = self.x * scalar;
    //     result.y = self.y * scalar;
    //     result.z = self.z * scalar;
    //     result.w = self.w * scalar;
    // }
    //
    // pub fn div_scalar(&self, scalar: f32, result: &mut V4) {
    //     let inv_scalar: f32 = 1.0 / scalar;
    //
    //     result.x = self.x * inv_scalar;
    //     result.y = self.y * inv_scalar;
    //     result.z = self.z * inv_scalar;
    //     result.w = self.w * inv_scalar;
    // }
    //
    // pub fn magnitude(&self) -> f32 {
    //     f32::sqrt(V4::dot_product(self, self))
    // }
    //
    // pub fn similarity(&self, other: &V4) -> f32 {
    //     V4::dot_product(self, other) / (self.magnitude() * other.magnitude())
    // }
    //
    // pub fn normalize(&self, result: &mut V4) {
    //     let x: f32 = self.x;
    //     let y: f32 = self.y;
    //     let z: f32 = self.z;
    //     let w: f32 = self.w;
    //
    //     let inv_mag: f32 = 1.0 / f32::sqrt(x * x + y * y + z * z + w * w);
    //
    //     result.x = x * inv_mag;
    //     result.y = y * inv_mag;
    //     result.z = z * inv_mag;
    //     result.w = w * inv_mag;
    // }
}
