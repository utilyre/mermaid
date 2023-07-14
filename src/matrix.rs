use crate::{
    identity::{IdAdd, IdMul},
    vec2::Vec2,
    vec3::Vec3,
};
use std::{
    array,
    fmt::{self, Display, Formatter},
    ptr,
};

pub mod aliases;
mod iter;
mod ops;

#[derive(Debug, Clone)]
pub struct Matrix<T, const M: usize, const N: usize>([[T; N]; M]);

impl<T, const M: usize, const N: usize> Matrix<T, M, N> {
    pub const fn new(rows: [[T; N]; M]) -> Self {
        Self(rows)
    }

    pub fn rows(&self) -> [[&T; N]; M] {
        array::from_fn(|i| array::from_fn(|j| &self[(i, j)]))
    }

    pub fn rows_mut(&mut self) -> [[&mut T; N]; M] {
        array::from_fn(|i| array::from_fn(|j| unsafe { &mut *(&mut self[(i, j)] as *mut T) }))
    }

    pub fn into_rows(self) -> [[T; N]; M] {
        self.0
    }

    pub fn row(&self, i: usize) -> Option<[&T; N]> {
        self.0.get(i).map(|row| array::from_fn(|j| &row[j]))
    }

    pub fn row_mut(&mut self, i: usize) -> Option<[&mut T; N]> {
        self.0
            .get_mut(i)
            .map(|row| array::from_fn(|j| unsafe { &mut *(&mut row[j] as *mut T) }))
    }

    pub fn take_row(self, i: usize) -> Option<[T; N]> {
        self.0.get(i).map(|row| unsafe { ptr::read(row) })
    }

    pub fn col(&self, j: usize) -> Option<[&T; M]> {
        (j < N).then(|| array::from_fn(|i| &self.0[i]).map(|row| &row[j]))
    }

    pub fn col_mut(&mut self, j: usize) -> Option<[&mut T; M]> {
        (j < N).then(|| {
            array::from_fn(|i| unsafe { &mut *(&mut self.0[i] as *mut [T; N]) })
                .map(|row| &mut row[j])
        })
    }

    pub fn take_col(self, j: usize) -> Option<[T; M]> {
        (j < N).then(|| array::from_fn::<_, M, _>(|i| unsafe { ptr::read(&self.0[i][j]) }))
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        self.0.get(i).and_then(|row| row.get(j))
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        self.0.get_mut(i).and_then(|row| row.get_mut(j))
    }

    pub fn take(self, i: usize, j: usize) -> Option<T> {
        self.0
            .get(i)
            .and_then(|row| row.get(j))
            .map(|x| unsafe { ptr::read(x) })
    }

    pub fn map<U, F>(&self, mut f: F) -> Matrix<U, M, N>
    where
        F: FnMut(usize, usize, &T) -> U,
    {
        Matrix::new(array::from_fn(|i| {
            array::from_fn(|j| f(i, j, &self[(i, j)]))
        }))
    }

    pub fn map_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, usize, &mut T),
    {
        for i in 0..M {
            for j in 0..N {
                f(i, j, &mut self[(i, j)]);
            }
        }
    }

    pub fn into_map<F>(mut self, mut f: F) -> Self
    where
        F: FnMut(usize, usize, T) -> T,
    {
        for i in 0..M {
            for j in 0..N {
                let x = f(i, j, unsafe { ptr::read(&self[(i, j)]) });
                unsafe {
                    ptr::write(&mut self[(i, j)], x);
                }
            }
        }

        self
    }
}

impl<T, const M: usize> Matrix<T, M, M> {
    pub fn diag(&self) -> [&T; M] {
        array::from_fn(|i| &self[(i, i)])
    }

    pub fn diag_mut(&mut self) -> [&mut T; M] {
        array::from_fn(|i| unsafe { &mut *(&mut self[(i, i)] as *mut T) })
    }

    pub fn into_diag(self) -> [T; M] {
        array::from_fn(|i| unsafe { ptr::read(&self[(i, i)]) })
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

impl<T, const M: usize> IdMul for Matrix<T, M, M>
where
    T: IdAdd + IdMul,
{
    fn id_mul() -> Self {
        Self::new(array::from_fn(|i| {
            array::from_fn(|j| if i == j { T::id_mul() } else { T::id_add() })
        }))
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

impl<T, const M: usize, const N: usize> Display for Matrix<T, M, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let strings = self.map(|_, _, x| x.to_string());
        let Some(max_len) = strings.iter().map(|x| x.len()).max() else {
            return write!(f, "||");
        };

        for i in 0..M {
            write!(f, "|")?;
            for j in 0..N {
                write!(f, "{: ^len$}", strings[(i, j)], len = max_len + 2)?;
            }
            write!(f, "|")?;

            if i < M - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_mut() {
        let mut mat = Matrix::new([[5, -1, 2], [-5, 0, -1]]);

        assert_eq!(
            [[&mut 5, &mut -1, &mut 2], [&mut -5, &mut 0, &mut -1]],
            mat.rows_mut()
        );
    }

    #[test]
    fn row_mut() {
        let mut mat = Matrix::new([[5, -1, 2], [-5, 0, -1]]);

        let row1 = mat.row_mut(0).unwrap();
        *row1[2] = 8;

        let row2 = mat.row_mut(1).unwrap();
        *row2[0] = 3;

        assert_eq!(&8, &mat[(0, 2)]);
        assert_eq!(&3, &mat[(1, 0)]);
    }

    #[test]
    fn take_row() {
        let mat = Matrix::new([[5, -1, 2], [-5, 0, -1]]);

        assert_eq!(Some([5, -1, 2]), mat.clone().take_row(0));
        assert_eq!(Some([-5, 0, -1]), mat.clone().take_row(1));
        assert_eq!(None, mat.clone().take_row(2));
        assert_eq!(None, mat.take_row(10));
    }

    #[test]
    fn col_mut() {
        let mut mat = Matrix::new([[5, -1, 2], [-5, 0, -1]]);

        let col1 = mat.col_mut(0).unwrap();
        *col1[1] = -2;

        let col2 = mat.col_mut(1).unwrap();
        *col2[0] = 0;

        let col3 = mat.col_mut(2).unwrap();
        *col3[1] = -8;

        assert_eq!(&-2, &mat[(1, 0)]);
        assert_eq!(&0, &mat[(0, 1)]);
        assert_eq!(&-8, &mat[(1, 2)]);
    }

    #[test]
    fn take_col() {
        let mat = Matrix::new([[5, -1, 2], [-5, 0, -1]]);

        assert_eq!(Some([5, -5]), mat.clone().take_col(0));
        assert_eq!(Some([-1, 0]), mat.clone().take_col(1));
        assert_eq!(Some([2, -1]), mat.clone().take_col(2));
        assert_eq!(None, mat.clone().take_col(3));
        assert_eq!(None, mat.take_col(8));
    }

    #[test]
    fn take() {
        let mat = Matrix::new([[4, -1, 3, 1], [-5, 8, 2, 0], [-3, -2, 1, 1]]);

        assert_eq!(Some(3), mat.clone().take(0, 2));
        assert_eq!(Some(8), mat.clone().take(1, 1));
        assert_eq!(Some(1), mat.clone().take(2, 3));
        assert_eq!(None, mat.clone().take(0, 4));
        assert_eq!(None, mat.take(3, 2));
    }

    #[test]
    fn into_map() {
        let mat = Matrix::new([[4, -1, 3, 1], [-5, 8, 2, 0], [-3, -2, 1, 1]]);

        assert_eq!(
            Matrix::new([[8, -2, 6, 2], [-10, 16, 4, 0], [-6, -4, 2, 2],]),
            mat.into_map(|_, _, x| 2 * x)
        )
    }

    #[test]
    fn diag_mut() {
        let mut mat = Matrix::new([[2, 5, 7], [6, 1, -1], [3, -2, 4]]);
        let diag = mat.diag_mut();
        *diag[0] = 1;
        *diag[1] = 7;
        *diag[2] = 6;

        assert_eq!([&1, &7, &6], mat.diag());
    }

    #[test]
    fn into_diag() {
        let mat = Matrix::new([[2, 5, 7], [6, 1, -1], [3, -2, 4]]);
        assert_eq!([2, 1, 4], mat.into_diag());
    }

    #[test]
    fn id_mul() {
        let mat3x3 = Matrix::<u32, 3, 3>::id_mul();
        let mat4x4 = Matrix::<u32, 4, 4>::id_mul();

        assert_eq!(Matrix::new([[1, 0, 0], [0, 1, 0], [0, 0, 1]]), mat3x3);
        assert_eq!(
            Matrix::new([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]),
            mat4x4
        );
    }

    #[test]
    fn display() {
        let mat = Matrix::new([[-1, 0], [2, 87], [-55, 3]]);

        assert_eq!(
            "\
            | -1    0  |\n\
            |  2   87  |\n\
            | -55   3  |\
            ",
            mat.to_string()
        );
    }
}
