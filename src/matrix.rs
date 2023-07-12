use crate::{
    identity::{IdAdd, IdMul},
    vec2::Vec2,
    vec3::Vec3,
};
use std::{
    array,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub type Matrix1x1<T> = Matrix<T, 1, 1>;
pub type Matrix1x2<T> = Matrix<T, 1, 2>;
pub type Matrix1x3<T> = Matrix<T, 1, 3>;

pub type Matrix2x1<T> = Matrix<T, 2, 1>;
pub type Matrix2x2<T> = Matrix<T, 2, 2>;
pub type Matrix2x3<T> = Matrix<T, 2, 3>;

pub type Matrix3x1<T> = Matrix<T, 3, 1>;
pub type Matrix3x2<T> = Matrix<T, 3, 2>;
pub type Matrix3x3<T> = Matrix<T, 3, 3>;

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

    pub fn map<F>(mut self, f: F) -> Self
    where
        F: Fn(usize, usize, &T) -> T,
    {
        for i in 0..M {
            for j in 0..N {
                self[(i, j)] = f(i, j, &self[(i, j)]);
            }
        }

        self
    }

    pub fn map_mut<F>(&mut self, f: F)
    where
        F: Fn(usize, usize, &mut T),
    {
        for i in 0..M {
            for j in 0..N {
                f(i, j, &mut self[(i, j)]);
            }
        }
    }

    pub fn scale<U, V>(&self, factor: U) -> Matrix<V, M, N>
    where
        for<'a, 'b> &'a T: Mul<&'b U, Output = V>,
        V: IdAdd,
    {
        Matrix::<V, M, N>::id_add().map(|i, j, _| &self[(i, j)] * &factor)
    }

    pub fn scale_mut<U>(&mut self, factor: U)
    where
        for<'a> T: MulAssign<&'a U>,
    {
        self.map_mut(|_, _, x| *x *= &factor);
    }
}

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

impl From<Vec2> for Matrix<f32, 2, 1> {
    fn from(value: Vec2) -> Self {
        Matrix::new([[value.x], [value.y]])
    }
}

impl<const N: usize> From<[Vec2; N]> for Matrix<f32, 2, N> {
    fn from(value: [Vec2; N]) -> Self {
        Matrix::new([
            array::from_fn(|i| value[i].x),
            array::from_fn(|i| value[i].y),
        ])
    }
}

impl From<Vec3> for Matrix<f32, 3, 1> {
    fn from(value: Vec3) -> Self {
        Matrix::new([[value.x], [value.y], [value.z]])
    }
}

impl<const N: usize> From<[Vec3; N]> for Matrix<f32, 3, N> {
    fn from(value: [Vec3; N]) -> Self {
        Matrix::new([
            array::from_fn(|i| value[i].x),
            array::from_fn(|i| value[i].y),
            array::from_fn(|i| value[i].z),
        ])
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

impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Matrix<T, M, N> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index.0, index.1).unwrap_or_else(|| {
            panic!(
                "index out of bounds: the len is ({}, {}) but the index is ({}, {})",
                M, N, index.0, index.1
            )
        })
    }
}

impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Matrix<T, M, N> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index.0, index.1).unwrap_or_else(|| {
            panic!(
                "index out of bounds: the len is ({}, {}) but the index is ({}, {})",
                M, N, index.0, index.1
            )
        })
    }
}

impl<T, U, V, const M: usize, const N: usize> Add<Matrix<U, M, N>> for Matrix<T, M, N>
where
    for<'a, 'b> &'a T: Add<&'b U, Output = V>,
    V: IdAdd,
{
    type Output = Matrix<V, M, N>;

    fn add(self, rhs: Matrix<U, M, N>) -> Self::Output {
        Self::Output::id_add().map(|i, j, _| &self[(i, j)] + &rhs[(i, j)])
    }
}

impl<T, U, const M: usize, const N: usize> AddAssign<Matrix<U, M, N>> for Matrix<T, M, N>
where
    for<'a> T: AddAssign<&'a U>,
{
    fn add_assign(&mut self, rhs: Matrix<U, M, N>) {
        self.map_mut(|i, j, x| *x += &rhs[(i, j)]);
    }
}

impl<T, U, V, const M: usize, const N: usize> Sub<Matrix<U, M, N>> for Matrix<T, M, N>
where
    for<'a, 'b> &'a T: Sub<&'b U, Output = V>,
    V: IdAdd,
{
    type Output = Matrix<V, M, N>;

    fn sub(self, rhs: Matrix<U, M, N>) -> Self::Output {
        Self::Output::id_add().map(|i, j, _| &self[(i, j)] - &rhs[(i, j)])
    }
}

impl<T, U, const M: usize, const N: usize> SubAssign<Matrix<U, M, N>> for Matrix<T, M, N>
where
    for<'a> T: SubAssign<&'a U>,
{
    fn sub_assign(&mut self, rhs: Matrix<U, M, N>) {
        self.map_mut(|i, j, x| *x -= &rhs[(i, j)]);
    }
}

impl<T, U, const M: usize, const N: usize> Neg for Matrix<T, M, N>
where
    for<'a> &'a T: Neg<Output = U>,
    U: IdAdd,
{
    type Output = Matrix<U, M, N>;

    fn neg(self) -> Self::Output {
        Self::Output::id_add().map(|i, j, _| -&self[(i, j)])
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
    fn scale() {
        let mat = new_matrix4x3_01();

        assert_eq!(
            Matrix::new([[40, 30, -5], [0, 35, 10], [20, 20, 25], [-15, -25, 15],]),
            mat.scale(5)
        );
    }

    #[test]
    fn scale_mut() {
        let mut m = new_matrix4x3_01();

        m.scale_mut(5);
        assert_eq!(
            Matrix::new([[40, 30, -5], [0, 35, 10], [20, 20, 25], [-15, -25, 15],]),
            m
        );
    }

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

    #[test]
    fn from_vec2() {
        let mat = Matrix::from([Vec2::new(9.0, 3.0), Vec2::new(2.0, 7.0)]);
        assert_eq!(Matrix::new([[9.0, 2.0], [3.0, 7.0],]), mat);
    }

    #[test]
    fn from_vec3() {
        let mat = Matrix::from([
            Vec3::new(9.0, 3.0, 4.0),
            Vec3::new(2.0, 7.0, 8.0),
            Vec3::new(5.0, 6.0, 0.0),
        ]);

        assert_eq!(
            Matrix::new([[9.0, 2.0, 5.0], [3.0, 7.0, 6.0], [4.0, 8.0, 0.0],]),
            mat
        );
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
}
