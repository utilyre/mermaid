use crate::{matrix::Matrix, prelude::IdAdd};
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        *self *= 1.0 / self.len();
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn proj(self, base: Self) -> Self {
        (self.dot(base) / base.len().powi(2)) * base
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

impl Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!(
                "index out of bounds: the len is 2 but the index is {}",
                index
            ),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!(
                "index out of bounds: the len is 2 but the index is {}",
                index
            ),
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(5.0, v.len());
    }

    #[test]
    fn normalize() {
        let mut v = Vec2::new(3.0, 4.0);

        v.normalize();
        assert_eq!(Vec2::new(0.6, 0.8), v);
    }

    #[test]
    fn dot() {
        let v1 = Vec2::new(1.0, 6.0);
        let v2 = Vec2::new(8.0, 9.0);

        assert_eq!(62.0, v1.dot(v2));
    }

    #[test]
    fn proj() {
        let v1 = Vec2::new(2.0, 5.0);
        let v2 = Vec2::new(3.0, 4.0);

        assert_eq!(Vec2::new(3.12, 4.16), v1.proj(v2));
    }

    #[test]
    fn add() {
        let v1 = Vec2::new(23.0, 18.0);
        let v2 = Vec2::new(5.0, 32.0);

        assert_eq!(Vec2::new(28.0, 50.0), v1 + v2);
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vec2::new(23.0, 18.0);
        let v2 = Vec2::new(5.0, 32.0);

        v1 += v2;
        assert_eq!(Vec2::new(28.0, 50.0), v1);
    }

    #[test]
    fn sub() {
        let v1 = Vec2::new(23.0, 18.0);
        let v2 = Vec2::new(5.0, 32.0);

        assert_eq!(Vec2::new(18.0, -14.0), v1 - v2);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Vec2::new(23.0, 18.0);
        let v2 = Vec2::new(5.0, 32.0);

        v1 -= v2;
        assert_eq!(Vec2::new(18.0, -14.0), v1);
    }

    #[test]
    fn neg() {
        let v = Vec2::new(-7.0, 13.0);
        assert_eq!(Vec2::new(7.0, -13.0), -v);
    }

    #[test]
    fn mul() {
        let v = Vec2::new(-7.0, 13.0);

        assert_eq!(Vec2::new(-10.5, 19.5), v * 1.5);
        assert_eq!(Vec2::new(-10.5, 19.5), 1.5 * v);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec2::new(-7.0, 13.0);

        v *= 1.5;
        assert_eq!(Vec2::new(-10.5, 19.5), v);
    }
}
