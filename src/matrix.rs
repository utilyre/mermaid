use crate::{
    identity::{AdditiveIdentity, MultiplicativeIdentity},
    vec2::Vec2,
};
use std::{
    array,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub const fn from_rows(rows: [[T; N]; M]) -> Self {
        Self(rows)
    }

    pub const fn from_cols(_cols: [[T; M]; N]) -> Self {
        // TODO: needs unsafe code
        unimplemented!()
    }

    pub fn into_rows(self) -> [[T; N]; M] {
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
        Matrix::from_rows([[value.x], [value.y]])
    }
}

impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
where
    T: Default,
{
    fn default() -> Self {
        Self::from_rows(array::from_fn(|_| array::from_fn(|_| T::default())))
    }
}

impl<T, const M: usize, const N: usize> AdditiveIdentity for Matrix<T, M, N>
where
    T: AdditiveIdentity,
{
    fn additive_identity() -> Self {
        Self::from_rows(array::from_fn(|_| {
            array::from_fn(|_| T::additive_identity())
        }))
    }
}

impl<T, const M: usize, const N: usize> MultiplicativeIdentity for Matrix<T, M, N>
where
    T: AdditiveIdentity + MultiplicativeIdentity,
{
    fn multiplicative_identity() -> Self {
        Self::from_rows(array::from_fn(|i| {
            array::from_fn(|j| {
                if i == j {
                    T::multiplicative_identity()
                } else {
                    T::additive_identity()
                }
            })
        }))
    }
}

impl<T, U, V, const M: usize, const N: usize> Add<Matrix<U, M, N>> for Matrix<T, M, N>
where
    for<'a, 'b> &'a T: Add<&'b U, Output = V>,
    V: Copy + AdditiveIdentity,
{
    type Output = Matrix<V, M, N>;

    fn add(self, rhs: Matrix<U, M, N>) -> Self::Output {
        let mut output = Self::Output::additive_identity();

        for i in 0..M {
            for j in 0..N {
                output.0[i][j] = &self.0[i][j] + &rhs.0[i][j];
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
                self.0[i][j] += &rhs.0[i][j];
            }
        }
    }
}

impl<T, U, V, const M: usize, const N: usize> Sub<Matrix<U, M, N>> for Matrix<T, M, N>
where
    for<'a, 'b> &'a T: Sub<&'b U, Output = V>,
    V: Copy + AdditiveIdentity,
{
    type Output = Matrix<V, M, N>;

    fn sub(self, rhs: Matrix<U, M, N>) -> Self::Output {
        let mut output = Self::Output::additive_identity();

        for i in 0..M {
            for j in 0..N {
                output.0[i][j] = &self.0[i][j] - &rhs.0[i][j];
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
                self.0[i][j] -= &rhs.0[i][j];
            }
        }
    }
}

impl<T, U, const M: usize, const N: usize> Neg for Matrix<T, M, N>
where
    for<'a> &'a T: Neg<Output = U>,
    U: Copy + AdditiveIdentity,
{
    type Output = Matrix<U, M, N>;

    fn neg(self) -> Self::Output {
        let mut output = Self::Output::additive_identity();

        for i in 0..M {
            for j in 0..N {
                output.0[i][j] = -&self.0[i][j];
            }
        }

        output
    }
}

impl<T, U, V, const M: usize, const N: usize> Mul<U> for Matrix<T, M, N>
where
    for<'a, 'b> &'a T: Mul<&'b U, Output = V>,
    V: Copy + AdditiveIdentity,
{
    type Output = Matrix<V, M, N>;

    fn mul(self, rhs: U) -> Self::Output {
        let mut output = Self::Output::additive_identity();

        for i in 0..M {
            for j in 0..N {
                output.0[i][j] = &self.0[i][j] * &rhs;
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
                self.0[i][j] *= &rhs;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn matrix_4x3_1() -> Matrix<i32, 4, 3> {
        Matrix::from_rows([
            [1,  2,  3 ],
            [4,  5,  6 ],
            [7,  8,  9 ],
            [10, 11, 12],
        ])
    }

    #[rustfmt::skip]
    fn matrix_4x3_2() -> Matrix<i32, 4, 3> {
        Matrix::from_rows([
            [1,  0,  5 ],
            [3,  2, -1 ],
            [3, -2,  7 ],
            [2,  0,  8 ],
        ])
    }

    #[test]
    fn row() {
        let m = matrix_4x3_1();

        assert_eq!(Some([&1, &2, &3]), m.row(0));
        assert_eq!(Some([&4, &5, &6]), m.row(1));
        assert_eq!(Some([&7, &8, &9]), m.row(2));
        assert_eq!(Some([&10, &11, &12]), m.row(3));
        assert_eq!(None, m.row(4));
        assert_eq!(None, m.row(5));
    }

    #[test]
    fn row_mut() {
        let mut m = matrix_4x3_1();

        assert_eq!(Some([&mut 1, &mut 2, &mut 3]), m.row_mut(0));
        assert_eq!(Some([&mut 4, &mut 5, &mut 6]), m.row_mut(1));
        assert_eq!(Some([&mut 7, &mut 8, &mut 9]), m.row_mut(2));
        assert_eq!(Some([&mut 10, &mut 11, &mut 12]), m.row_mut(3));
        assert_eq!(None, m.row_mut(4));
        assert_eq!(None, m.row_mut(5));
    }

    #[test]
    fn col() {
        let m = matrix_4x3_1();

        assert_eq!(Some([&1, &4, &7, &10]), m.col(0));
        assert_eq!(Some([&2, &5, &8, &11]), m.col(1));
        assert_eq!(Some([&3, &6, &9, &12]), m.col(2));
        assert_eq!(None, m.col(3));
        assert_eq!(None, m.col(4));
    }

    #[test]
    fn col_mut() {
        let mut m = matrix_4x3_1();

        assert_eq!(Some([&mut 1, &mut 4, &mut 7, &mut 10]), m.col_mut(0));
        assert_eq!(Some([&mut 2, &mut 5, &mut 8, &mut 11]), m.col_mut(1));
        assert_eq!(Some([&mut 3, &mut 6, &mut 9, &mut 12]), m.col_mut(2));
        assert_eq!(None, m.col_mut(3));
        assert_eq!(None, m.col_mut(4));
    }

    #[test]
    fn get() {
        let m = matrix_4x3_1();

        assert_eq!(Some(&1), m.get(0, 0));
        assert_eq!(Some(&4), m.get(1, 0));
        assert_eq!(Some(&12), m.get(3, 2));
        assert_eq!(None, m.get(4, 0));
        assert_eq!(None, m.get(2, 5));
        assert_eq!(None, m.get(6, 7));
    }

    #[test]
    fn get_mut() {
        let mut m = matrix_4x3_1();

        assert_eq!(Some(&mut 1), m.get_mut(0, 0));
        assert_eq!(Some(&mut 4), m.get_mut(1, 0));
        assert_eq!(Some(&mut 12), m.get_mut(3, 2));
        assert_eq!(None, m.get_mut(4, 0));
        assert_eq!(None, m.get_mut(2, 5));
        assert_eq!(None, m.get_mut(6, 7));
    }

    #[test]
    fn identity() {
        let m_3x3 = Matrix::<u32, 3, 3>::multiplicative_identity();
        let m_4x4 = Matrix::<u32, 4, 4>::multiplicative_identity();

        assert_eq!(Matrix::from_rows([[1, 0, 0], [0, 1, 0], [0, 0, 1]]), m_3x3);
        assert_eq!(
            Matrix::from_rows([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]),
            m_4x4
        );
    }

    #[test]
    fn add() {
        let m1 = matrix_4x3_1();
        let m2 = matrix_4x3_2();

        assert_eq!(
            Matrix::from_rows([[2, 2, 8], [7, 7, 5], [10, 6, 16], [12, 11, 20]]),
            m1 + m2
        );
    }

    #[test]
    fn add_assign() {
        let mut m1 = matrix_4x3_1();
        let m2 = matrix_4x3_2();

        m1 += m2;

        assert_eq!(
            Matrix::from_rows([[2, 2, 8], [7, 7, 5], [10, 6, 16], [12, 11, 20]]),
            m1
        )
    }

    #[test]
    fn sub() {
        let m1 = matrix_4x3_1();
        let m2 = matrix_4x3_2();

        assert_eq!(
            Matrix::from_rows([[0, 2, -2], [1, 3, 7], [4, 10, 2], [8, 11, 4]]),
            m1 - m2
        );
    }

    #[test]
    fn sub_assign() {
        let mut m1 = matrix_4x3_1();
        let m2 = matrix_4x3_2();

        m1 -= m2;

        assert_eq!(
            Matrix::from_rows([[0, 2, -2], [1, 3, 7], [4, 10, 2], [8, 11, 4]]),
            m1
        )
    }

    #[test]
    fn neg() {
        let m = matrix_4x3_2();

        assert_eq!(
            Matrix::from_rows([[-1, 0, -5], [-3, -2, 1], [-3, 2, -7], [-2, 0, -8],]),
            -m
        );
    }

    #[test]
    fn mul() {
        let k = 5;
        let m = matrix_4x3_1();

        assert_eq!(
            Matrix::from_rows([[5, 10, 15], [20, 25, 30], [35, 40, 45], [50, 55, 60],]),
            m * k
        );
    }

    #[test]
    fn mul_assign() {
        let k = 5;
        let mut m = matrix_4x3_1();

        m *= k;

        assert_eq!(
            Matrix::from_rows([[5, 10, 15], [20, 25, 30], [35, 40, 45], [50, 55, 60],]),
            m
        );
    }
}
