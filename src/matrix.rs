use crate::{
    identity::{IdAdd, IdMul},
    vec2::Vec2,
};
use std::{
    array,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub const fn new(rows: [[T; N]; M]) -> Self {
        Self(rows)
    }

    pub fn into_inner(self) -> [[T; N]; M] {
        self.0
    }

    pub fn row(&self, i: usize) -> Option<[&T; N]> {
        self.0.get(i).map(|row| row.each_ref())
    }

    pub fn row_mut(&mut self, i: usize) -> Option<[&mut T; N]> {
        self.0.get_mut(i).map(|row| row.each_mut())
    }

    pub fn col(&self, j: usize) -> Option<[&T; M]> {
        (j < N).then(|| self.0.each_ref().map(|row| &row[j]))
    }

    pub fn col_mut(&mut self, j: usize) -> Option<[&mut T; M]> {
        (j < N).then(|| self.0.each_mut().map(|row| &mut row[j]))
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        self.0.get(i).and_then(|row| row.get(j))
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        self.0.get_mut(i).and_then(|row| row.get_mut(j))
    }
}

impl From<Vec2> for Matrix<f32, 2, 1> {
    fn from(value: Vec2) -> Self {
        Matrix::new([[value.x], [value.y]])
    }
}

impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(array::from_fn(|_| array::from_fn(|_| T::default())))
    }
}

impl<T, const M: usize, const N: usize> IdAdd for Matrix<T, M, N>
where
    T: IdAdd,
{
    fn id_add() -> Self {
        Self::new(array::from_fn(|_| array::from_fn(|_| T::id_add())))
    }
}

impl<T, const M: usize, const N: usize> IdMul for Matrix<T, M, N>
where
    T: IdAdd + IdMul,
{
    fn id_mul() -> Self {
        Self::new(array::from_fn(|i| {
            array::from_fn(|j| if i == j { T::id_mul() } else { T::id_add() })
        }))
    }
}

impl<T, U, V, const M: usize, const N: usize> Add<Matrix<U, M, N>> for Matrix<T, M, N>
where
    for<'a, 'b> &'a T: Add<&'b U, Output = V>,
    V: IdAdd,
{
    type Output = Matrix<V, M, N>;

    fn add(self, rhs: Matrix<U, M, N>) -> Self::Output {
        let mut output = Self::Output::id_add();

        for i in 0..M {
            for j in 0..N {
                *output.get_mut(i, j).expect("is not out of bounds") =
                    self.get(i, j).expect("is not out of bounds")
                        + rhs.get(i, j).expect("is not out of bounds");
            }
        }

        output
    }
}

impl<T, U, const M: usize, const N: usize> AddAssign<Matrix<U, M, N>> for Matrix<T, M, N>
where
    for<'a> T: AddAssign<&'a U>,
{
    fn add_assign(&mut self, rhs: Matrix<U, M, N>) {
        for i in 0..M {
            for j in 0..N {
                *self.get_mut(i, j).expect("is not out of bounds") +=
                    rhs.get(i, j).expect("is not out of bounds");
            }
        }
    }
}

impl<T, U, V, const M: usize, const N: usize> Sub<Matrix<U, M, N>> for Matrix<T, M, N>
where
    for<'a, 'b> &'a T: Sub<&'b U, Output = V>,
    V: IdAdd,
{
    type Output = Matrix<V, M, N>;

    fn sub(self, rhs: Matrix<U, M, N>) -> Self::Output {
        let mut output = Self::Output::id_add();

        for i in 0..M {
            for j in 0..N {
                *output.get_mut(i, j).expect("is not out of bounds") =
                    self.get(i, j).expect("is not out of bounds")
                        - rhs.get(i, j).expect("is not out of bounds");
            }
        }

        output
    }
}

impl<T, U, const M: usize, const N: usize> SubAssign<Matrix<U, M, N>> for Matrix<T, M, N>
where
    for<'a> T: SubAssign<&'a U>,
{
    fn sub_assign(&mut self, rhs: Matrix<U, M, N>) {
        for i in 0..M {
            for j in 0..N {
                *self.get_mut(i, j).expect("is not out of bounds") -=
                    rhs.get(i, j).expect("is not out of bounds");
            }
        }
    }
}

impl<T, U, const M: usize, const N: usize> Neg for Matrix<T, M, N>
where
    for<'a> &'a T: Neg<Output = U>,
    U: IdAdd,
{
    type Output = Matrix<U, M, N>;

    fn neg(self) -> Self::Output {
        let mut output = Self::Output::id_add();

        for i in 0..M {
            for j in 0..N {
                *output.get_mut(i, j).expect("is not out of bounds") =
                    -self.get(i, j).expect("is not out of bounds");
            }
        }

        output
    }
}

impl<T, U, V, const M: usize, const N: usize> Mul<U> for Matrix<T, M, N>
where
    for<'a, 'b> &'a T: Mul<&'b U, Output = V>,
    V: IdAdd,
{
    type Output = Matrix<V, M, N>;

    fn mul(self, rhs: U) -> Self::Output {
        let mut output = Self::Output::id_add();

        for i in 0..M {
            for j in 0..N {
                *output.get_mut(i, j).expect("is not out of bounds") =
                    self.get(i, j).expect("is not out of bounds") * &rhs;
            }
        }

        output
    }
}

impl<T, U, const M: usize, const N: usize> MulAssign<U> for Matrix<T, M, N>
where
    for<'a> T: MulAssign<&'a U>,
{
    fn mul_assign(&mut self, rhs: U) {
        for i in 0..M {
            for j in 0..N {
                *self.get_mut(i, j).expect("is not out of bounds") *= &rhs;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn new_matrix4x3_01() -> Matrix<i32, 4, 3> {
        Matrix::new([
            [ 8,  6, -1],
            [ 0,  7,  2],
            [ 4,  4,  5],
            [-3, -5,  3],
        ])
    }

    #[rustfmt::skip]
    fn new_matrix4x3_02() -> Matrix<i32, 4, 3> {
        Matrix::new([
            [1,  0,  5 ],
            [3,  2, -1 ],
            [3, -2,  7 ],
            [2,  0,  8 ],
        ])
    }

    #[test]
    fn row_mut() {
        let mut m = new_matrix4x3_01();

        let row = m.row_mut(2).unwrap();
        *row[1] = 10;
        *row[2] = 8;

        assert_eq!(Some(&10), m.get(2, 1));
        assert_eq!(Some(&8), m.get(2, 2));
    }

    #[test]
    fn col_mut() {
        let mut m = new_matrix4x3_01();

        let col = m.col_mut(2).unwrap();
        *col[0] = 1;
        *col[3] = 5;

        assert_eq!(Some(&1), m.get(0, 2));
        assert_eq!(Some(&5), m.get(3, 2));
    }

    #[test]
    fn id_mul() {
        let m_3x3 = Matrix::<u32, 3, 3>::id_mul();
        let m_4x4 = Matrix::<u32, 4, 4>::id_mul();

        assert_eq!(Matrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]), m_3x3);
        assert_eq!(
            Matrix::new([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]),
            m_4x4
        );
    }

    #[test]
    fn add() {
        let mat1 = new_matrix4x3_01();
        let mat2 = new_matrix4x3_02();

        assert_eq!(
            Matrix::new([[9, 6, 4], [3, 9, 1], [7, 2, 12], [-1, -5, 11]]),
            mat1 + mat2
        );
    }

    #[test]
    fn add_assign() {
        let mut mat1 = new_matrix4x3_01();
        let mat2 = new_matrix4x3_02();

        mat1 += mat2;
        assert_eq!(
            Matrix::new([[9, 6, 4], [3, 9, 1], [7, 2, 12], [-1, -5, 11]]),
            mat1
        )
    }

    #[test]
    fn sub() {
        let mat1 = new_matrix4x3_01();
        let mat2 = new_matrix4x3_02();

        assert_eq!(
            Matrix::new([[7, 6, -6], [-3, 5, 3], [1, 6, -2], [-5, -5, -5]]),
            mat1 - mat2
        );
    }

    #[test]
    fn sub_assign() {
        let mut mat1 = new_matrix4x3_01();
        let mat2 = new_matrix4x3_02();

        mat1 -= mat2;
        assert_eq!(
            Matrix::new([[7, 6, -6], [-3, 5, 3], [1, 6, -2], [-5, -5, -5]]),
            mat1
        )
    }

    #[test]
    fn neg() {
        let mat = new_matrix4x3_02();

        assert_eq!(
            Matrix::new([[-1, 0, -5], [-3, -2, 1], [-3, 2, -7], [-2, 0, -8],]),
            -mat
        );
    }

    #[test]
    fn mul() {
        let mat = new_matrix4x3_01();

        assert_eq!(
            Matrix::new([[40, 30, -5], [0, 35, 10], [20, 20, 25], [-15, -25, 15],]),
            mat * 5
        );
    }

    #[test]
    fn mul_assign() {
        let mut m = new_matrix4x3_01();

        m *= 5;
        assert_eq!(
            Matrix::new([[40, 30, -5], [0, 35, 10], [20, 20, 25], [-15, -25, 15],]),
            m
        );
    }
}
