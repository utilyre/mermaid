use crate::{matrix::Matrix, prelude::IdAdd};

mod ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const I: Self = Self::new(1.0, 0.0);
    pub const J: Self = Self::new(0.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl IdAdd for Vec2 {
    fn id_add() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(value: [f32; 2]) -> Self {
        Self::new(value[0], value[1])
    }
}

impl From<Matrix<f32, 2, 1>> for Vec2 {
    fn from(value: Matrix<f32, 2, 1>) -> Self {
        Self::new(value[(0, 0)], value[(1, 0)])
    }
}
