use super::Matrix;
use std::ops::{Add, Mul, Sub};

impl<T> Matrix<T, 1, 1>
where
    T: ToOwned,
{
    pub fn det(&self) -> T::Owned {
        self[(0, 0)].to_owned()
    }
}

impl<T> Matrix<T, 2, 2>
where
    for<'a, 'b> &'a T: Mul<&'b T, Output = T>,
    T: Sub<T, Output = T>,
{
    pub fn det(&self) -> T {
        &self[(0, 0)] * &self[(1, 1)] - &self[(0, 1)] * &self[(1, 0)]
    }
}

impl<T> Matrix<T, 3, 3>
where
    for<'a, 'b> &'a T: Mul<&'b T, Output = T>,
    for<'a> T: Add<T, Output = T> + Sub<T, Output = T> + Mul<&'a T, Output = T>,
{
    pub fn det(&self) -> T {
        &self[(0, 0)] * &self[(1, 1)] * &self[(2, 2)]
            + &self[(0, 1)] * &self[(1, 2)] * &self[(2, 0)]
            + &self[(0, 2)] * &self[(1, 0)] * &self[(2, 1)]
            - &self[(0, 2)] * &self[(1, 1)] * &self[(2, 0)]
            - &self[(0, 0)] * &self[(1, 2)] * &self[(2, 1)]
            - &self[(0, 1)] * &self[(1, 0)] * &self[(2, 2)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn det1x1() {
        let mat = Matrix::new([[-3]]);
        assert_eq!(-3, mat.det());
    }

    #[test]
    fn det2x2() {
        let mat = Matrix::new([[5, -1], [8, 2]]);
        assert_eq!(18, mat.det());
    }

    #[test]
    fn det3x3() {
        let mat = Matrix::new([[0, -1, 2], [3, 2, 1], [-2, -3, 4]]);
        assert_eq!(4, mat.det());
    }
}
