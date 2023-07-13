use super::Matrix;
use crate::{identity::IdAdd, recip::Recip};
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::ptr;

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub fn scale<U, V>(&self, factor: U) -> Matrix<V, M, N>
    where
        for<'a, 'b> &'a T: Mul<&'b U, Output = V>,
        V: IdAdd,
    {
        Matrix::<V, M, N>::id_add().into_map(|i, j, _| &self[(i, j)] * &factor)
    }

    pub fn scale_mut<U>(&mut self, factor: U)
    where
        for<'a> T: MulAssign<&'a U>,
    {
        self.map_mut(|_, _, x| *x *= &factor);
    }
}

impl<T, const M: usize> Matrix<T, M, M>
where
    for<'a, 'b> &'a T: Mul<&'b T, Output = T>,
    T: Clone + Add<T, Output = T> + IdAdd,
{
    pub fn pow(self, exp: u32) -> Self {
        (0..exp).fold(self.clone(), |acc, _| acc * self.clone())
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
        Self::Output::id_add().into_map(|i, j, _| &self[(i, j)] + &rhs[(i, j)])
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
        Self::Output::id_add().into_map(|i, j, _| &self[(i, j)] - &rhs[(i, j)])
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
        Self::Output::id_add().into_map(|i, j, _| -&self[(i, j)])
    }
}

impl<T, U, V, W, const M: usize, const N: usize, const P: usize> Mul<Matrix<U, P, N>>
    for Matrix<T, M, P>
where
    for<'a, 'b> &'a T: Mul<&'b U, Output = V>,
    V: Add<V, Output = W>,
    W: Add<V, Output = W> + IdAdd,
{
    type Output = Matrix<W, M, N>;

    fn mul(self, rhs: Matrix<U, P, N>) -> Self::Output {
        Self::Output::id_add().into_map(|i, j, mut x| {
            for k in 0..P {
                x = x + &self[(i, k)] * &rhs[(k, j)];
            }

            x
        })
    }
}

impl<T, U> Recip for Matrix<T, 1, 1>
where
    T: Recip<Output = U>,
{
    type Output = Matrix<U, 1, 1>;

    fn recip(self) -> Self {
        Self::new([[unsafe { ptr::read(&self[(0, 0)]) }.recip()]])
    }
}

impl<T, U> Recip for Matrix<T, 2, 2>
where
    for<'a, 'b> &'a T: Mul<&'b T, Output = T>,
    T: Sub<T, Output = T> + Neg<Output = T> + Recip<Output = U> + IdAdd,
{
    type Output = Matrix<U, 2, 2>;

    fn recip(self) -> Self {
        let factor = self.det().recip();

        Self::new([
            [
                unsafe { ptr::read(&self[(1, 1)]) },
                unsafe { ptr::read(&self[(0, 1)]) }.neg(),
            ],
            [unsafe { ptr::read(&self[(1, 0)]) }.neg(), unsafe {
                ptr::read(&self[(0, 0)])
            }],
        ])
        .scale(factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale() {
        let mut mat = Matrix::new([[8, 6, -1], [0, 7, 2], [4, 4, 5], [-3, -5, 3]]);

        assert_eq!(
            Matrix::new([[40, 30, -5], [0, 35, 10], [20, 20, 25], [-15, -25, 15],]),
            mat.clone().scale(5)
        );

        mat.scale_mut(5);
        assert_eq!(
            Matrix::new([[40, 30, -5], [0, 35, 10], [20, 20, 25], [-15, -25, 15],]),
            mat
        )
    }

    #[test]
    fn pow() {
        let mat = Matrix::new([[1, 3], [2, -1]]);
        assert_eq!(Matrix::new([[49, 0], [0, 49]]), mat.pow(3));
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
    fn add() {
        let mut mat1 = Matrix::new([[8, 6, -1], [0, 7, 2], [4, 4, 5], [-3, -5, 3]]);
        let mat2 = Matrix::new([[1, 0, 5], [3, 2, -1], [3, -2, 7], [2, 0, 8]]);

        assert_eq!(
            Matrix::new([[9, 6, 4], [3, 9, 1], [7, 2, 12], [-1, -5, 11]]),
            mat1.clone() + mat2.clone()
        );

        mat1 += mat2;
        assert_eq!(
            Matrix::new([[9, 6, 4], [3, 9, 1], [7, 2, 12], [-1, -5, 11]]),
            mat1
        );
    }

    #[test]
    fn sub() {
        let mut mat1 = Matrix::new([[8, 6, -1], [0, 7, 2], [4, 4, 5], [-3, -5, 3]]);
        let mat2 = Matrix::new([[1, 0, 5], [3, 2, -1], [3, -2, 7], [2, 0, 8]]);

        assert_eq!(
            Matrix::new([[7, 6, -6], [-3, 5, 3], [1, 6, -2], [-5, -5, -5]]),
            mat1.clone() - mat2.clone()
        );

        mat1 -= mat2;
        assert_eq!(
            Matrix::new([[7, 6, -6], [-3, 5, 3], [1, 6, -2], [-5, -5, -5]]),
            mat1
        );
    }

    #[test]
    fn neg() {
        let mat = Matrix::new([[1, 0, 5], [3, 2, -1], [3, -2, 7], [2, 0, 8]]);

        assert_eq!(
            Matrix::new([[-1, 0, -5], [-3, -2, 1], [-3, 2, -7], [-2, 0, -8],]),
            -mat
        );
    }

    #[test]
    fn mul() {
        let mat1 = Matrix::new([[8, 6, -1], [0, 7, 2], [4, 4, 5], [-3, -5, 3]]);
        let mat2 = Matrix::new([[3, -1], [2, -3], [-2, 1]]);

        assert_eq!(
            Matrix::new([[38, -27], [10, -19], [10, -11], [-25, 21],]),
            mat1 * mat2
        );
    }

    #[test]
    fn inverse1x1() {
        let mat = Matrix::new([[5.0]]);
        assert_eq!(Matrix::new([[0.2]]), mat.recip());
    }

    #[test]
    fn inverse2x2() {
        let mat = Matrix::new([[5.0, 6.0], [-5.0, 2.0]]);
        assert_eq!(40.0, mat.det());
        assert_eq!(
            Matrix::new([[0.05, -0.15], [0.125, 0.125]]),
            mat.recip().into_map(|_, _, x| (1000.0_f64 * x).trunc() / 1000.0)
        );
    }
}
