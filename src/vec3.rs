use crate::{matrix::Matrix, prelude::IdAdd};

mod ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const I: Self = Self::new(1.0, 0.0, 0.0);
    pub const J: Self = Self::new(0.0, 1.0, 0.0);
    pub const K: Self = Self::new(0.0, 0.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl IdAdd for Vec3 {
    fn id_add() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(value: [f32; 3]) -> Self {
        Self::new(value[0], value[1], value[2])
    }
}

impl From<Matrix<f32, 3, 1>> for Vec3 {
    fn from(value: Matrix<f32, 3, 1>) -> Self {
        Self::new(value[(0, 0)], value[(1, 0)], value[(2, 0)])
    }
}
