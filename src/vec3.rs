use crate::{matrix::Matrix, prelude::IdAdd};
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
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

    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&mut self) {
        *self *= 1.0 / self.len();
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn proj(self, base: Self) -> Self {
        (self.dot(base) / base.len().powi(2)) * base
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

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!(
                "index out of bounds: the len is 2 but the index is {}",
                index
            ),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!(
                "index out of bounds: the len is 2 but the index is {}",
                index
            ),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let v = Vec3::new(3.0, 4.0, 12.0);
        assert_eq!(13.0, v.len());
    }

    #[test]
    fn normalize() {
        let mut v = Vec3::new(3.0, 0.0, 4.0);

        v.normalize();
        assert_eq!(Vec3::new(0.6, 0.0, 0.8), v);
    }

    #[test]
    fn dot() {
        let v1 = Vec3::new(1.0, 6.0, 3.0);
        let v2 = Vec3::new(8.0, 9.0, 2.0);

        assert_eq!(68.0, v1.dot(v2));
    }

    #[test]
    fn proj() {
        let v1 = Vec3::new(2.0, 5.0, 7.0);
        let v2 = Vec3::new(0.0, 3.0, 4.0);

        assert_eq!(Vec3::new(0.0, 5.16, 6.88), v1.proj(v2));
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 6.0, 3.0);
        let v2 = Vec3::new(8.0, 9.0, 2.0);

        assert_eq!(Vec3::new(-15.0, 22.0, -39.0), v1.cross(v2));
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(23.0, 18.0, 1.0);
        let v2 = Vec3::new(5.0, 32.0, 12.0);

        assert_eq!(Vec3::new(28.0, 50.0, 13.0), v1 + v2);
    }

    #[test]
    fn add_assign() {
        let mut v1 = Vec3::new(23.0, 18.0, 1.0);
        let v2 = Vec3::new(5.0, 32.0, 12.0);

        v1 += v2;
        assert_eq!(Vec3::new(28.0, 50.0, 13.0), v1);
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(23.0, 18.0, 1.0);
        let v2 = Vec3::new(5.0, 32.0, 12.0);

        assert_eq!(Vec3::new(18.0, -14.0, -11.0), v1 - v2);
    }

    #[test]
    fn sub_assign() {
        let mut v1 = Vec3::new(23.0, 18.0, 1.0);
        let v2 = Vec3::new(5.0, 32.0, 12.0);

        v1 -= v2;
        assert_eq!(Vec3::new(18.0, -14.0, -11.0), v1);
    }

    #[test]
    fn neg() {
        let v = Vec3::new(-7.0, 13.0, 2.0);
        assert_eq!(Vec3::new(7.0, -13.0, -2.0), -v);
    }

    #[test]
    fn mul() {
        let v = Vec3::new(-7.0, 13.0, 2.0);

        assert_eq!(Vec3::new(-10.5, 19.5, 3.0), v * 1.5);
        assert_eq!(Vec3::new(-10.5, 19.5, 3.0), 1.5 * v);
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::new(-7.0, 13.0, 2.0);

        v *= 1.5;
        assert_eq!(Vec3::new(-10.5, 19.5, 3.0), v);
    }
}
