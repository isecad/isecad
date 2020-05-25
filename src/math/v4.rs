#![allow(dead_code)]

use crate::*;

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
}

impl Zero<V4> for V4 {
    fn zero() -> V4 {
        V4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
}
